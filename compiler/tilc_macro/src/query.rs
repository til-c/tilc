mod kw {
  use syn::custom_keyword;

  custom_keyword!(query);
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{
  Attribute, Error, Expr, Ident, Pat, Result, ReturnType, Token, Type, braced,
  parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
  spanned::Spanned,
};


struct Query {
  doc_comments: Vec<Attribute>,
  modifiers: QueryModifiers,
  name: Ident,
  key: Pat,
  arg: Type,
  result: ReturnType,
}
impl Parse for Query {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let doc_comments = {
      let attrs = input.call(Attribute::parse_outer)?;
      let f = |attr: Attribute| {
        if !attr.path().is_ident("doc") {
          return Err(Error::new(
            attr.span(),
            "Supports only doc comment attribute",
          ));
        } else {
          return Ok(attr);
        }
      };

      attrs.into_iter().map(f).collect::<Result<Vec<_>>>()?
    };


    input.parse::<kw::query>()?;
    let name = input.parse::<Ident>()?;
    let args;
    parenthesized!(args in input);
    let key = Pat::parse_single(&args)?;
    args.parse::<Token![:]>()?;
    let arg = args.parse::<Type>()?;
    args.parse::<Option<Token![,]>>()?;
    let result = input.parse::<ReturnType>()?;


    let content;
    braced!(content in input);
    let modifiers = content.parse::<QueryModifiers>()?;


    return Ok(Self {
      doc_comments,
      modifiers,
      name,
      key,
      arg,
      result,
    });
  }
}
struct QueryModifiers {
  desc: (Option<Ident>, Punctuated<Expr, Token![,]>),
  arena_cache: Option<Ident>,
  cache: Option<Ident>,
  feedable: Option<Ident>,
}
impl Parse for QueryModifiers {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let mut desc = None;
    let mut arena_cache = None;
    let mut cache = None;
    let mut feedable = None;


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

      match modifier.to_string().as_str() {
        "desc" => {
          let modifier_content;
          braced!(modifier_content in input);
          let tcxt = if modifier_content.peek(Token![|]) {
            modifier_content.parse::<Token![|]>()?;
            let tcxt = modifier_content.parse::<Ident>()?;
            modifier_content.parse::<Token![|]>()?;
            Some(tcxt)
          } else {
            None
          };

          let list =
            modifier_content.parse_terminated(Expr::parse, Token![,])?;
          try_insert!(desc = (tcxt, list));
        }
        "arena_cache" => try_insert!(arena_cache = modifier),
        "cache" => try_insert!(cache = modifier),
        "feedable" => try_insert!(feedable = modifier),
        _ => {
          return Err(Error::new(modifier.span(), "unknown query modifier"));
        }
      };
    }
    let Some(desc) = desc else {
      return Err(Error::new(input.span(), "no desc provided"));
    };

    return Ok(Self {
      desc,
      arena_cache,
      cache,
      feedable,
    });
  }
}
struct List<T>(Vec<T>);
impl<T: Parse> Parse for List<T> {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let mut list = Vec::new();
    while !input.is_empty() {
      list.push(input.parse()?);
    }
    return Ok(List(list));
  }
}

pub(crate) fn tilc_queries(input: TokenStream) -> TokenStream {
  let queries = parse_macro_input!(input as List<Query>);

  let mut query_stream = quote! {};
  let mut descs_queries = quote! {};
  let mut feedable_queries = quote! {};

  for query in queries.0 {
    let Query {
      doc_comments,
      modifiers,
      name,
      key,
      arg,
      result,
    } = query;
    let result = match result {
      ReturnType::Default => quote! { -> ()},
      _ => quote! { #result },
    };

    let mut attributes = Vec::new();

    if let Some(arena_cache) = modifiers.arena_cache {
      attributes.push(arena_cache);
    };


    query_stream.extend(quote! {
      #(#doc_comments)*
      [#(#attributes),*]
      fn #name(#arg) #result,
    });

    if let Some(_) = modifiers.feedable {
      feedable_queries.extend(quote! {
        #(#doc_comments)*
        [#(#attributes),*] fn #name(#arg) #result,
      });
    };


    let (tcx, desc) = modifiers.desc;
    let tcx = tcx.map_or_else(
      || {
        quote! { _ }
      },
      |tcx| quote! { #tcx },
    );
    descs_queries.extend(quote! {
      pub fn #name<'ctxt>(tcx: TyCtxt<'ctxt>, key: crate::query::queries::#name::Key<'ctxt>) -> String {
        let (#tcx, #key) = (tcx, key);
        return format!(#desc);
      }
    });
  }


  return TokenStream::from(quote! {
    #[macro_export]
    macro_rules! all_queries {
      ($macro:ident!) => {
        $macro! {
          #query_stream
        }
      };
    }

    macro_rules! feedable_queries {
      ($macro:ident!) => {
        $macro! {
          #feedable_queries
        }
      };
    }

    pub mod descs {
      pub use super::*;
      #descs_queries
    }
  });
}
