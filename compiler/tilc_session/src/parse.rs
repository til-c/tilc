use std::collections::HashMap;

use tilc_span::{Edition, Span, Symbol};


pub struct SymbolRepo {
  pub symbols: HashMap<Symbol, Span>,
}
pub struct ParseSession {
  pub edition: Edition,

  pub symbol_repo: SymbolRepo,
}
