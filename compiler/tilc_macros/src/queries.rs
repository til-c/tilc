use proc_macro::TokenStream;
use quote::quote;
use syn::{
  Attribute, Error, Ident, Pat, Result, ReturnType, Token, Type, braced, parenthesized,
  parse::{Parse, ParseStream},
  parse_macro_input,
};

mod kw {
  syn::custom_keyword!(query);
}

struct Query {
  attrs: Vec<Attribute>,
  name: Ident,

  key: Pat,
  arg: Type,

  return_ty: ReturnType,

  modifiers: QueryModifier,
}
impl Parse for Query {
  fn parse(input: ParseStream) -> Result<Self> {
    let attrs = Attribute::parse_outer(input)?;
    input.parse::<kw::query>()?;
    let name = input.parse::<Ident>()?;

    let args;
    parenthesized!(args in input);
    let key = Pat::parse_single(&args)?;
    args.parse::<Token![:]>()?;
    let arg = args.parse::<Type>()?;

    let return_ty = input.parse::<ReturnType>()?;

    let _modifiers;
    braced!(_modifiers in input);
    let modifiers = _modifiers.parse::<QueryModifier>()?;

    Ok(Self {
      attrs,
      name,

      key,
      arg,

      return_ty,

      modifiers,
    })
  }
}
struct QueryModifier {
  feedable: Option<Ident>,
  cache: Option<Ident>,
}
impl Parse for QueryModifier {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut feedable = None;
    let mut cache = None;

    while !input.is_empty() {
      let modifier = input.parse::<Ident>()?;
      macro_rules! try_insert {
        ($name:ident = $expr:expr) => {
          if $name.is_some() {
            return Err(Error::new(modifier.span(), "duplicate modifer"));
          } else {
            $name = Some($expr);
          }
        };
      }

      match &*modifier.to_string() {
        "feedable" => try_insert!(feedable = modifier),
        "cache" => try_insert!(cache = modifier),

        _ => {
          return Err(Error::new(modifier.span(), "unknown query modifier"));
        }
      };
    }
    Ok(Self { feedable, cache })
  }
}

struct List<T>(Vec<T>);
impl<T> Parse for List<T>
where
  T: Parse,
{
  fn parse(input: ParseStream) -> Result<Self> {
    let mut list = Vec::new();
    while !input.is_empty() {
      list.push(input.parse()?);
    }
    Ok(Self(list))
  }
}
pub(super) fn queries(input: TokenStream) -> TokenStream {
  let queries = parse_macro_input!(input as List<Query>);

  let mut query_stream = quote! {};
  let mut feedable_query_stream = quote! {};

  for Query {
    attrs,
    name,

    key,
    arg,

    return_ty,

    modifiers,
  } in queries.0
  {
    let return_ty = match return_ty {
      ReturnType::Default => quote! { -> () },
      _ => quote! { #return_ty },
    };

    let mut modifier_stream = Vec::new();
    macro_rules! through_modifiers {
      ($(
        $modifier:ident,
      )*) => {$(
        if let Some(m) = &modifiers.$modifier {
          modifier_stream.push(quote! { (#m) })
        };
      )*};
    }
    through_modifiers! {
      feedable,
      cache,
    }

    query_stream.extend(quote! {
      #(#attrs)*
      [#(#modifier_stream)*] fn #name(#arg) #return_ty,
    });
    if let Some(_) = &modifiers.feedable {
      feedable_query_stream.extend(quote! {
        #(#attrs)*
        [#(#modifier_stream)*] fn #name(#arg) #return_ty,
      });
    };
  }
  // panic!("{}", query_stream.to_string());
  eprintln!("{}", query_stream.to_string());

  TokenStream::from(quote! {
    macro_rules! all_queries {
      ($macro:ident!) => {
        $macro! { #query_stream }
      };
    }
    macro_rules! feedable_queries {
      ($macro:ident!) => {
        $macro! { #feedable_query_stream }
      };
    }
  })
}
