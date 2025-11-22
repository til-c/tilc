use proc_macro::TokenStream;

mod symbols;
mod uidx;

#[proc_macro]
pub fn uidx(input: TokenStream) -> TokenStream {
  return uidx::uidx(input);
}
#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
  return symbols::symbols(input);
}
