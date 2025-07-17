use std::fmt::Debug;

use tilc_ast::{NodeIdx, Sandyq};
use tilc_session::Session;


pub trait ResolverExpander {
  fn next_node_idx(&mut self) -> NodeIdx;
}
impl<'a> Debug for dyn ResolverExpander + 'a {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f.write_str(&format!("{:?}", self));
  }
}

#[derive(Debug)]
pub struct ExpansiontCtxt<'a> {
  pub session: &'a Session,

  pub resolver: &'a mut dyn ResolverExpander,
}
impl<'a> ExpansiontCtxt<'a> {
  pub fn new(
    session: &'a Session,
    resolver: &'a mut dyn ResolverExpander,
  ) -> Self {
    return Self { session, resolver };
  }
  pub fn monotonic_expander<'b>(
    &'b mut self,
    is_monotonic: bool,
  ) -> MacroExpander<'b, 'a> {
    return MacroExpander {
      expansion_ctxt: self,
      is_monotonic,
    };
  }
}

#[derive(Debug)]
pub struct MacroExpander<'a, 'b> {
  expansion_ctxt: &'a mut ExpansiontCtxt<'b>,
  is_monotonic: bool,
}
impl<'a, 'b> MacroExpander<'a, 'b> {
  pub fn expand_sandyq(&mut self, sandyq: Sandyq) -> Sandyq {
    todo!();
  }
}
