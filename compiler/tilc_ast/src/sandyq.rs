use std::sync::atomic::{AtomicU32, Ordering};

use tilc_macro::uidx;
use tilc_span::{Ident, Span, Symbol};

use crate::{Delim, DelimSpan, FnCtxt, FnKind, Lit, TokenStream, WalkItemKind};


uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
  pub struct NodeIdx {
    const DUMMY = Self::MAX.as_u32();
  }
}
pub const SANDYQ_NODE_IDX: NodeIdx = NodeIdx::from_u32(0);

uidx! {
  #[derive(Debug, Clone, Copy)]
  pub struct AttrIdx {}
}


#[derive(Debug, Clone)]
pub struct Sandyq {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub items: Vec<Item>,

  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Attribute {
  pub idx: AttrIdx,

  pub path: Path,
  pub args: AttrArgs,

  pub style: AttributeStyle,
  pub span: Span,
}
#[derive(Debug, Clone, Copy)]
pub enum AttributeStyle {
  Inner,
  Outer,
}
#[derive(Debug, Clone)]
pub enum AttrArgs {
  Empty,
  Delimited(DelimArgs),
  Eq {
    eq_span: Span,
    expr: Box<Expression>,
  },
}

#[derive(Debug, Clone)]
pub struct Item<K = ItemKind> {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub kind: K,
  pub ident: Ident,

  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
  Static(Box<Static>),
  Const(Box<Const>),
  Use(Box<Use>),
  Fn(Box<Fn>),
  Korpe(Safety, Ident, Korpe),
  ForeignKorpe(ForeignKorpe),
  TyAlias(Box<TyAlias>),
  Enum(Ident, Generics, Vec<EnumVariant>),
  Struct(Ident, Generics, VariantKind),
  Union(Ident, Generics, VariantKind),
  Trait(Box<Trait>),
  Impl(Box<Impl>),
  MacroCall(Box<MacroCall>),
  MacroDef(Ident, MacroDef),
}
#[derive(Debug, Clone)]
pub enum ForeignItemKind {
  Static(Box<Static>),
  Fn(Box<Fn>),
}
#[derive(Debug, Clone)]
pub enum AssociatedItemKind {
  Const(Box<Const>),
  Fn(Box<Fn>),
  TyAlias(Box<TyAlias>),
}

#[derive(Debug, Clone)]
pub struct MacroDef {
  pub body: DelimArgs,
  pub is_macro_rules: bool,
}
#[derive(Debug, Clone)]
pub struct MacroCall {
  pub ident: Ident,
  pub args: DelimArgs,
}
#[derive(Debug, Clone)]
pub struct Impl {
  pub safety: Safety,
  pub generics: Generics,
  pub kind: ImplKind,
  pub self_ty: Box<Ty>,
  pub items: Vec<Item<AssociatedItemKind>>,

  pub defaultness: Defaultness,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum ImplKind {
  Positive,
  Negative(Span),
}

#[derive(Debug, Clone)]
pub struct Trait {
  pub idx: NodeIdx,

  pub vis: Visibility,
  pub safety: Safety,
  pub ident: Ident,
  pub items: Vec<Item<AssociatedItemKind>>,

  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub ident: Ident,

  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum VariantKind {
  Struct(Vec<FieldDef>),
  Tuple(Vec<FieldDef>),
  Unit,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub ident: Option<Ident>,
  pub ty: Box<Ty>,

  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct TyAlias {
  pub vis: Visibility,
  pub ident: Ident,
  pub generics: Generics,
  pub ty: Option<Box<Ty>>,

  pub defaultness: Defaultness,
}
#[derive(Debug, Clone)]
pub struct Static {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub mutability: Mutability,
  pub ident: Ident,
  pub ty: Box<Ty>,
  pub expr: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Const {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub ident: Ident,
  pub generics: Generics,
  pub ty: Box<Ty>,
  pub expr: Box<Expression>,

  pub defaultness: Defaultness,
}


#[derive(Debug, Clone)]
pub struct ForeignKorpe {
  pub extern_span: Span,

  pub safety: Safety,
  pub abi: Option<Symbol>,
  pub items: Vec<Item<ForeignItemKind>>,
}
#[derive(Debug, Clone)]
pub enum Korpe {
  Braced(Vec<Item>, Span),
  File,
}
#[derive(Debug, Clone)]
pub struct Path {
  pub segments: Vec<PathSegment>,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct PathSegment {
  pub ident: Ident,
  pub idx: NodeIdx,
}

#[derive(Debug, Clone)]
pub struct Fn {
  pub generics: Generics,
  pub ident: Ident,
  pub fn_sig: FnSig,
  pub block: Option<Block>,
}
#[derive(Debug, Clone)]
pub struct FnSig {
  pub fn_header: FnHeader,
  pub fn_decl: FnDecl,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct FnHeader {
  pub is_const: bool,
  pub is_async: bool,
  // TODO: Implement async and extern features
}
#[derive(Debug, Clone)]
pub struct FnDecl {
  pub params: Vec<Param>,
  pub return_ty: FnReturnType,
}
#[derive(Debug, Clone)]
pub enum FnReturnType {
  Default,
  Other(Box<Ty>),
}
#[derive(Debug, Clone)]
pub struct Param {
  pub idx: NodeIdx,

  pub ty: Box<Ty>,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Generics {
  pub params: Vec<GenericParam>,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct GenericParam {
  pub idx: NodeIdx,
  pub ident: Ident,
  pub kind: GenericParamKind,
}
#[derive(Debug, Clone)]
pub enum GenericParamKind {
  Lifetime,
  Type,
  Const,
}


#[derive(Debug, Clone)]
pub struct Ty {
  pub idx: NodeIdx,

  pub kind: TyKind,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum TyKind {
  /// No-return type
  ///
  /// !
  Never,

  /// Autodetect type
  ///
  /// _
  Infer,

  /// Reference type
  ///
  /// [`&'a T`], [`&'a auspaly T`], [`&T`], [`&auspaly T`]
  // Ref(Option<Box<Lifetime>>, Box<MutTy>),

  // TODO:
  /// Pointer type
  Ptr(),

  /// Fixed array size
  ///
  /// [b8; 2]
  // Array(Box<Ty>, Box<Const>),

  /// Non fixed array size
  ///
  /// [b8]
  Slice(Box<Ty>),

  /// Tuple
  ///
  /// (b8, b16, b32)
  Tuple(Vec<Ty>),

  /// ```til
  /// qurylum Alma {};
  /// ```
  /// Alma
  Path(Box<Path>),
}


#[derive(Debug, Clone)]
pub struct Block {
  pub idx: NodeIdx,

  pub statements: Vec<Statement>,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Statement {
  pub idx: NodeIdx,

  pub kind: StatementKind,

  pub span: Span,
}
impl Statement {
  pub fn access_attrs<F: FnMut(&mut Vec<Attribute>)>(&mut self, f: F) {
    self.kind.access_attrs(f);
  }
}
#[derive(Debug, Clone)]
pub enum StatementKind {
  Let(Box<Local>),
  Item(Box<Item>),
  Expression(Box<Expression>),
  Semi(Box<Expression>),
}
impl StatementKind {
  fn access_attrs<F: FnMut(&mut Vec<Attribute>)>(&mut self, mut f: F) {
    match self {
      Self::Let(local) => f(&mut local.attrs),
      Self::Item(item) => f(&mut item.attrs),
      Self::Expression(expr) | Self::Semi(expr) => f(&mut expr.attrs),
    };
  }
}
#[derive(Debug, Clone)]
pub struct Expression {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub kind: ExpressionKind,

  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum ExpressionKind {
  /// ainymaly $pat = $expr
  Let(Box<Pattern>, Box<Expression>, Span),
  /// $lit
  Lit(Lit),
  /// $expr = $expr
  Assign(Box<Expression>, Span, Box<Expression>),
}
#[derive(Debug, Clone)]
pub struct Local {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub pat: Box<Pattern>,
  pub kind: LocalKind,
  pub ty: Option<Box<Ty>>,

  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum LocalKind {
  Decl,
  Init(Box<Expression>),
}
#[derive(Debug, Clone)]
pub struct Pattern {
  pub idx: NodeIdx,

  pub kind: PatternKind,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum PatternKind {
  /// $mut? $ident $pat
  Ident(Mutability, Ident, Option<Box<Pattern>>),

  /// ($pat, $pat, ...)
  Tuple(Vec<Box<Pattern>>),

  /// $struct {$arg, $arg, ...}
  Struct(Path, Vec<FieldDef>),
  /// $expr
  Expression(Box<Expression>),
  /// &$mut? $pat
  Ref(Mutability, Box<Pattern>),
}

#[derive(Debug, Clone)]
pub struct Use {
  pub prefix: Box<Path>,
  pub kind: UseKind,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum UseKind {
  Single(Option<Ident>),
  Multiple(Vec<UseKind>),

  Everything,
}

#[derive(Debug, Clone)]
pub struct Visibility {
  pub kind: VisibilityKind,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub enum VisibilityKind {
  Public,
  Private,
  Protected(NodeIdx, Box<Path>),
}

#[derive(Debug, Clone)]
pub struct DelimArgs {
  pub delim: Delim,
  pub span: DelimSpan,
  pub tokens: TokenStream,
}


#[derive(Debug, Clone)]
pub enum Safety {
  Safe(Span),
  Unsafe(Span),
  Inherit,
}
#[derive(Debug, Clone)]
pub enum Mutability {
  Mut,
  Nope,
}
#[derive(Debug, Clone)]
pub enum Defaultness {
  Default,
  Overriden,
}


#[derive(Debug)]
pub struct AttrIdxGen(AtomicU32);
impl AttrIdxGen {
  pub const fn new() -> Self {
    return Self(AtomicU32::new(0));
  }
  pub fn make_attr_idx(&self) -> AttrIdx {
    let next_idx = self.0.fetch_add(1, Ordering::Relaxed);
    assert!(next_idx != u32::MAX);
    return AttrIdx(next_idx);
  }
}


impl WalkItemKind for ItemKind {
  fn mut_walk<W: crate::Walker>(
    &mut self,
    span: Span,
    idx: NodeIdx,
    vis: &mut Visibility,
    walker: &mut W,
  ) {
    match self {
      Self::Fn(fx) => {
        let fn_kind = FnKind::Fn(FnCtxt::Free, vis, fx);
        walker.mut_walk_fn(fn_kind);
      }
      _ => todo!(),
    };
  }
  fn walk<W: crate::Walker>(
    &self,
    span: Span,
    idx: NodeIdx,
    vis: &Visibility,
    walker: &mut W,
  ) {
    todo!();
  }
}

macro_rules! impl_has_attrs {
  ($(
    $ident:ident $(<$T:ident>)?,
  )*) => {$(
    impl HasAttrs for $ident $(<$T>)? {
      fn attrs(&self) -> &[Attribute] {
        return &self.attrs;
      }
      fn walk_attrs<F>(&mut self, f: F)
      where
        F: FnOnce(&mut Vec<Attribute>), {
          f(&mut self.attrs);
        }
    })*
  };
}
pub trait HasAttrs {
  fn attrs(&self) -> &[Attribute];
  fn walk_attrs<F>(&mut self, f: F)
  where
    F: FnOnce(&mut Vec<Attribute>);
}
impl_has_attrs! {
  Sandyq,
  Item,
  Item<ForeignItemKind>,
  Item<AssociatedItemKind>,
  Expression,
}
impl HasAttrs for StatementKind {
  fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Let(local) => return &local.attrs,
      Self::Item(item) => return item.attrs(),
      Self::Expression(expr) | Self::Semi(expr) => return expr.attrs(),
    };
  }

  fn walk_attrs<F>(&mut self, f: F)
  where
    F: FnOnce(&mut Vec<Attribute>), {
    match self {
      Self::Let(local) => return f(&mut local.attrs),
      Self::Item(item) => return item.walk_attrs(f),
      Self::Expression(expr) | Self::Semi(expr) => return expr.walk_attrs(f),
    };
  }
}
impl HasAttrs for Statement {
  fn attrs(&self) -> &[Attribute] {
    return self.kind.attrs();
  }
  fn walk_attrs<F>(&mut self, f: F)
  where
    F: FnOnce(&mut Vec<Attribute>), {
    self.kind.walk_attrs(f);
  }
}

macro_rules! impl_has_node_idx {
  ($(
    $ident:ident $(<$T:ident>)?,
  )*) => {$(
    impl HasNodeIdx for $ident $(<$T>)? {
      fn node_idx(&self) -> NodeIdx {
        return self.idx;
      }
      fn mut_node_idx(&mut self) -> &mut NodeIdx {
        return &mut self.idx;
      }
    }
  )*};
}
pub trait HasNodeIdx {
  fn node_idx(&self) -> NodeIdx;
  fn mut_node_idx(&mut self) -> &mut NodeIdx;
}
impl_has_node_idx! {
  Sandyq,
  Item,
  Item<ForeignItemKind>,
  Item<AssociatedItemKind>,
  Expression,
}
