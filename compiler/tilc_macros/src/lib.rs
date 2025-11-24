mod queries;
mod symbols;
mod uidx;

use proc_macro::TokenStream;

#[proc_macro]
pub fn uidx(input: TokenStream) -> TokenStream {
  uidx::uidx(input)
}
#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
  symbols::symbols(input)
}

#[proc_macro]
pub fn queries(input: TokenStream) -> TokenStream {
  queries::queries(input)
}
