mod symbol;

use proc_macro::TokenStream;


#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
  return symbol::symbols(input.into()).into();
}
