mod diagnostic;
mod query;
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

#[proc_macro]
pub fn tilc_queries(input: TokenStream) -> TokenStream {
  return query::tilc_queries(input);
}

#[proc_macro_derive(Diagnostic, attributes(diag, warning, note))]
pub fn diagnostic(input: TokenStream) -> TokenStream {
  return diagnostic::diagnostic(input);
}
