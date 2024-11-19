use tilc_macros::symbols;

#[cfg(test)]
mod test {
  use quote::quote;
  use syn::{
    braced,
    parse::{self, Parse},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr, Token,
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


  #[test]
  fn symbols() {
    let input = quote! {
      Keywords {
        Function: "fx",
        Let: "ainymaly",
        Const: "turaqty",
      }
    };
    println!("{}", input);

    let input: Input = match parse2(input.into()) {
      Ok(input) => input,

      Err(err) => panic!("{}", err),
    };
    println!("{:?}", input);
  }
}
