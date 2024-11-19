use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
  braced,
  parse::{self, Parse},
  parse2,
  punctuated::Punctuated,
  token::Comma,
  Error, Ident, LitStr, Token,
};


struct Keyword {
  name: Ident,
  value: LitStr,
}
impl std::fmt::Debug for Keyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f
      .debug_struct("Keyword")
      .field("name", &self.name)
      .field("value", &self.value.value())
      .finish();
  }
}
impl Parse for Keyword {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    let name: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let value: LitStr = input.parse()?;

    Ok(Keyword { name, value })
  }
}
struct Input {
  keywords: Punctuated<Keyword, Comma>,
}
impl std::fmt::Debug for Input {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut f = f.debug_list();

    for keyword in self.keywords.iter() {
      f.entry(&keyword);
    }

    return f.finish();
  }
}
impl Parse for Input {
  fn parse(input: parse::ParseStream) -> syn::Result<Self> {
    input.parse::<Ident>()?;
    // println!("{}", keywords);
    let content;
    braced!(content in input);
    let keywords: Punctuated<Keyword, Comma> =
      Punctuated::parse_terminated(&content)?;

    return Ok(Input { keywords });
  }
}

pub fn symbols(input: TokenStream) -> TokenStream {
  let input: Input = match parse2(input.into()) {
    Ok(input) => input,

    // FIXME: Implement proper error handling for parsing error
    Err(err) => panic!("{}", err),
  };

  let mut keywords: TokenStream = quote! {};
  let mut entries: Entries = Entries::with_capacity(input.keywords.len());


  for keyword in input.keywords.iter() {
    let name: &Ident = &keyword.name;
    let value: &LitStr = &keyword.value;
    let idx: u32 = entries.insert(&value.value(), name.span());
    keywords.extend(quote! {
      pub const #name: Symbol = Symbol::new_m(#idx);
    });
  }


  let pre_interned_len: u32 = entries.len();
  return quote! {
    pub const PRE_INTERNED_SYMBOLS_LEN: u32 = #pre_interned_len;


    #[allow(non_upper_case_globals, unused)]
    mod kw {
      use super::Symbol;
      #keywords
    }
  };
}


struct Errors(Vec<Error>);
impl Errors {}


struct Interned {
  idx: u32,
  span: Span,
}
struct Entries(HashMap<String, Interned>);
impl Entries {
  pub fn with_capacity(capacity: usize) -> Self {
    return Self(HashMap::with_capacity(capacity));
  }


  pub fn insert(&mut self, key: &str, span: Span) -> u32 {
    if let Some(prev) = self.0.get(key) {
      eprintln!("Symbol {} already exists", key);
      eprintln!("Symbol span: {:?}", prev.span);

      return prev.idx;
    } else {
      let idx: u32 = self.len();
      self.0.insert(key.to_string(), Interned { idx, span });

      return idx;
    };
  }


  pub fn len(&self) -> u32 {
    return u32::try_from(self.0.len()).expect("Too many symbols");
  }
}
