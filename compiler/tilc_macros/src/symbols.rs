mod kw {
  syn::custom_keyword!(Keywords);
  syn::custom_keyword!(Symbols);
}

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Ident, LitStr, Result, Token, braced,
  parse::{Parse, ParseStream},
  parse_macro_input,
  punctuated::Punctuated,
};

struct Keyword {
  name: Ident,
  value: LitStr,
}
impl Parse for Keyword {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse()?;
    input.parse::<Token![:]>()?;
    let value = input.parse()?;
    Ok(Self { name, value })
  }
}
struct Symbol {
  name: Ident,
  value: LitStr,
}
impl Parse for Symbol {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse()?;
    input.parse::<Token![:]>()?;
    let value = input.parse()?;
    Ok(Self { name, value })
  }
}

struct Input {
  keywords: Punctuated<Keyword, Token![,]>,
  symbols: Punctuated<Symbol, Token![,]>,
}
impl Parse for Input {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<kw::Keywords>()?;
    let content;
    braced!(content in input);
    let keywords = Punctuated::parse_terminated(&content)?;

    input.parse::<kw::Symbols>()?;
    let content;
    braced!(content in input);
    let symbols = Punctuated::parse_terminated(&content)?;

    Ok(Self { keywords, symbols })
  }
}

struct Interned {
  idx: u32,
  span: Span,
}
struct Entries(HashMap<String, Interned>);
impl Entries {
  fn with_capacity(cap: usize) -> Self {
    Self(HashMap::with_capacity(cap))
  }

  fn insert(&mut self, key: &str, span: Span) -> u32 {
    if let Some(prev) = self.0.get(key) {
      eprintln!("Entry {} already exists", key);
      eprintln!("Entry span: {:?}", prev.span);

      prev.idx
    } else {
      let idx = self.len();
      self.0.insert(key.to_string(), Interned { idx, span });
      idx
    }
  }
  fn len(&self) -> u32 {
    u32::try_from(self.0.len()).expect("too many entries")
  }
}

pub(super) fn symbols(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as Input);

  let mut keywords = quote! {};
  let mut symbols = quote! {};
  let mut interner_prefill = quote! {};
  let mut entries = Entries::with_capacity(input.keywords.len() + input.symbols.len() + 1);

  for keyword in input.keywords.iter() {
    let name = &keyword.name;
    let value = &keyword.value;
    let idx = entries.insert(&value.value(), name.span());

    interner_prefill.extend(quote! {
      #value,
    });

    keywords.extend(quote! {
      pub const #name: Symbol = Symbol::new(#idx);
    });
  }
  for symbol in input.symbols.iter() {
    let name = &symbol.name;
    let value = &symbol.value;
    let idx = entries.insert(&value.value(), name.span());

    interner_prefill.extend(quote! {
      #value,
    });

    symbols.extend(quote! {
      pub const #name: Symbol = Symbol::new(#idx);
    });
  }

  let pre_interned_len = entries.len();
  TokenStream::from(quote! {
    pub const PRE_INTERNED_SYMBOLS_LEN: u32 = #pre_interned_len;

    #[allow(non_upper_case_globals)]
    pub mod kw {
      use crate::Symbol;
      #keywords
    }
    #[allow(non_upper_case_globals)]
    pub mod sym {
      use crate::Symbol;
      #symbols
    }

    impl crate::Interner {
      pub fn with_prefilled() -> Self {
        Self::prefill(&[#interner_prefill])
      }
    }
  })
}
