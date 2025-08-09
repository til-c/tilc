use std::fmt::Debug;

use tilc_ast::NodeIdx;

use crate::AstFragment;


pub trait ResolverExpander {
  fn next_node_idx(&mut self) -> NodeIdx;

  fn def_colletor<'a>(&mut self, ast_fragment: &'a AstFragment);
}
impl<'a> Debug for dyn ResolverExpander + 'a {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f.write_str(&format!("{:?}", self));
  }
}
