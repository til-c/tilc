mod symbols;
mod uidx;

use proc_macro::TokenStream;


#[proc_macro]
pub fn uidx(input: TokenStream) -> TokenStream {
  return uidx::uidx(input);
}

#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
  return symbols::symbols(input);
}
