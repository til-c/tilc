use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Attribute, Expr, Ident, Lit, LitInt, Meta, MetaNameValue, Token, Visibility,
  braced, parse::Parse, parse_macro_input,
};


struct Uidx(TokenStream);
impl Parse for Uidx {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut attrs = input.call(Attribute::parse_outer)?;
    let vis: Visibility = input.parse()?;
    input.parse::<Token![struct]>()?;
    let name: Ident = input.parse()?;

    let body;
    braced!(body in input);

    let mut max = None;
    let mut consts = Vec::new();


    attrs.retain(|attr| match attr.path().get_ident() {
      Some(ident) => match ident.to_string().as_str() {
        "max" => {
          let Meta::NameValue(MetaNameValue {
            value: Expr::Lit(lit),
            ..
          }) = &attr.meta
          else {
            panic!("#[max = NUMBER] attribute requires max value");
          };
          if let Some(old) = max.replace(lit.lit.clone()) {
            panic!("Specified multiple max: {:?}", old);
          };

          false
        }
        _ => true,
      },

      None => true,
    });

    loop {
      if body.is_empty() {
        break;
      };

      let const_attrs = body.call(Attribute::parse_outer)?;
      body.parse::<Token![const]>()?;
      let const_name: Ident = body.parse()?;
      body.parse::<Token![=]>()?;
      let const_value: Expr = body.parse()?;
      body.parse::<Token![;]>()?;
      consts.push(quote! {
        #(#const_attrs)*
        #vis const #const_name: #name = #name::from_u32(#const_value);
      });
    }
    let max = max.unwrap_or_else(|| {
      Lit::Int(LitInt::new("0xFFFF_FF00", Span::call_site()))
    });

    return Ok(Uidx(TokenStream::from(quote! {
      #(#attrs)*
      #vis struct #name(u32);

      impl #name {
        #(#consts)*
        #vis const MAX: Self = Self::from_u32(#max);

        #vis const EMPTY: Self = Self::from_u32(0);


        #[inline]
        #vis const fn from_u16(v: u16) -> Self {
          assert!(v as u32 <= #max);
          unsafe {
            return Self::from_u32_unchecked(v as u32);
          };
        }
        #[inline]
        #vis const fn from_u32(v: u32) -> Self {
          assert!(v <= #max);
          unsafe {
            return Self::from_u32_unchecked(v);
          };
        }
        #[inline]
        #vis const fn from_usize(v: usize) -> Self {
          assert!(v <= (#max as usize));
          unsafe {
            return Self::from_u32_unchecked(v as u32);
          };
        }


        #[inline]
        #vis const fn as_u16(self) -> u16 {
          return self.0 as u16;
        }
        #[inline]
        #vis const fn as_u32(self) -> u32 {
          return self.0;
        }
        #[inline]
        #vis const fn as_usize(self) -> usize {
            return self.as_u32() as usize;
        }

        #[inline]
        const unsafe fn from_u32_unchecked(v: u32) -> Self {
          return Self(v);
        }
      }
      impl ::std::ops::Add<usize> for #name {
        type Output = Self;

        fn add(self, rhs: usize) -> Self::Output {
          return Self::from_usize(self.as_usize() + rhs);
        }
      }
      impl ::std::ops::Sub<usize> for #name {
        type Output = Self;

        fn sub(self, rhs: usize) -> Self {
          return Self::from_usize(self.as_usize() - rhs);
        }
      }

      impl From<#name> for u32 {
        #[inline]
        fn from(v: #name) -> u32 {
         return v.as_u32();
        }
      }

      impl From<#name> for usize {
        #[inline]
        fn from(v: #name) -> usize {
          return v.as_usize();
        }
      }

      impl From<usize> for #name {
        #[inline]
        fn from(v: usize) -> Self {
         return Self::from_usize(v);
        }
      }

      impl From<u32> for #name {
        #[inline]
        fn from(v: u32) -> Self {
         return Self::from_u32(v);
        }
      }
    })));
  }
}

pub(crate) fn uidx(input: TokenStream) -> TokenStream {
  let uidx = parse_macro_input!(input as Uidx);
  return uidx.0;
}
