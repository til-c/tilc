use std::fmt::Debug;

use tilc_ast::NodeIdx;


pub trait ResolverExpander {
  fn next_node_idx(&mut self) -> NodeIdx;
}
impl<'a> Debug for dyn ResolverExpander + 'a {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f.write_str(&format!("{:?}", self));
  }
}
