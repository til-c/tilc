mod uidx;

use proc_macro::TokenStream;


#[proc_macro]
pub fn uidx(input: TokenStream) -> TokenStream {
  return uidx::uidx(input);
}
