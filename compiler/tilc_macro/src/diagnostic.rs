use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};


pub(crate) fn diagnostic(input: TokenStream) -> TokenStream {
  let structure = parse_macro_input!(input as ItemStruct);

  return TokenStream::from(quote! {});
}
