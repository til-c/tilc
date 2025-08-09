use tilc_ast::{
  Attribute, HasAttrs, HasNodeIdx, Item, MacroCall, NodeIdx, Sandyq, Statement,
  Walker, mut_walk_sandyq,
};
use tilc_session::Session;
use tilc_span::Span;

use crate::ResolverExpander;


macro_rules! ast_fragment {
  ($(
    $kind:ident($ast_ty:ty) {
      $kind_name:expr;
      $(walker fn $mut_walk_ast:ident; fn $walk_ast:ident;)?
      fn $make_ast:ident;
    }
  )*) => {
    #[derive(Clone)]
    pub enum AstFragment {$(
      $kind($ast_ty),
    )*}
    impl AstFragment {
      fn mut_walk_ast<W: Walker>(&mut self, walker: &mut W) {
        match self {
          $($(Self::$kind(ast) => walker.$mut_walk_ast(ast),)?)*
        };
      }
      pub fn walk_ast<'a, W: Walker>(&'a self, walker: &mut W) {
        match self {
          $($(Self::$kind(ast) => walker.$walk_ast(ast),)?)*
        }
      }

      $(pub fn $make_ast(self) -> $ast_ty {
        match self {
          Self::$kind(ast) => return ast,
          _ => panic!("make_ast called on wrong AstFragment"),
        };
      })*
    }

    #[derive(Debug)]
    pub enum AstFragmentKind {$(
      $kind,
    )*}
  };
}
ast_fragment! {
  Sandyq(Sandyq) {
    "sandyq";
    walker fn mut_walk_sandyq; fn walk_sandyq;
    fn make_sandyq;
  }
  Item(Box<Item>) {
    "item";
    walker fn mut_walk_item; fn walk_item;
    fn make_items;
  }
  Statement(Box<Statement>) {
    "statement";
    walker fn mut_walk_stmt; fn walk_stmt;
    fn make_statement;
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
      ecx: self,
      is_monotonic,
    };
  }
}

#[derive(Debug)]
pub struct MacroExpander<'a, 'b> {
  pub ecx: &'a mut ExpansiontCtxt<'b>,
  is_monotonic: bool,
}
impl<'a, 'b> MacroExpander<'a, 'b> {
  pub fn expand_sandyq(&mut self, sandyq: Sandyq) -> Sandyq {
    let mut fragment = AstFragment::Sandyq(sandyq);
    let mut collector = MacroInvocationCollector {
      ecx: self.ecx,
      invocation: Vec::new(),
      is_monotonic: self.is_monotonic,
    };

    fragment.mut_walk_ast(&mut collector);


    if self.is_monotonic {
      self.ecx.resolver.def_colletor(&fragment);
    };

    return fragment.make_sandyq();
  }
}


struct MacroInvocationCollector<'a, 'b> {
  pub ecx: &'a mut ExpansiontCtxt<'b>,
  invocation: Vec<Invocation>,
  is_monotonic: bool,
}
impl<'a, 'b> MacroInvocationCollector<'a, 'b> {
  pub fn pick_attr(&mut self, node: &mut impl HasAttrs) -> Option<Attribute> {
    let mut attr = None;
    node.walk_attrs(|attrs| {
      if attrs.is_empty() {
        return;
      };
      attr = Some(attrs.remove(0));
    });

    return attr;
  }

  fn set_idx<Node: InvocationCollectorNode>(&mut self, node: &mut Node) {
    if self.is_monotonic {
      let new_idx = self.ecx.resolver.next_node_idx();
      *node.mut_node_idx() = new_idx;
    };
    node.walk(self);
  }
  fn visit_node<Node: InvocationCollectorNode>(&mut self, node: &mut Node) {
    match self.pick_attr(node) {
      Some(attr) => todo!("{:?}", attr),
      None => self.set_idx(node),
    }
  }
}
impl<'a, 'b> Walker for MacroInvocationCollector<'a, 'b> {
  fn mut_walk_sandyq(&mut self, sandyq: &mut Sandyq) {
    self.visit_node(sandyq);
  }

  fn mut_walk_idx(&mut self, idx: &mut tilc_ast::NodeIdx) {
    if self.is_monotonic && *idx == NodeIdx::DUMMY {
      *idx = self.ecx.resolver.next_node_idx();
    };
  }
}

#[derive(Debug)]
pub struct Invocation {
  kind: InvocationKind,
  fragment_kind: AstFragmentKind,
}
#[derive(Debug)]
pub enum InvocationKind {
  Bang {
    macro_call: Box<MacroCall>,
    span: Span,
  },
  Attr {
    attr: Attribute,
  },
}

trait InvocationCollectorNode: HasAttrs + HasNodeIdx {
  fn walk<W: Walker>(&mut self, walker: &mut W);
}

impl InvocationCollectorNode for Sandyq {
  fn walk<W: Walker>(&mut self, walker: &mut W) {
    mut_walk_sandyq(walker, self);
  }
}
