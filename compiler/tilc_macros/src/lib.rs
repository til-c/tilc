use proc_macro::TokenStream;

mod symbols;
mod uidx;

#[proc_macro]
pub fn uidx(input: TokenStream) -> TokenStream {
  uidx::uidx(input)
}
#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
  symbols::symbols(input)
}
