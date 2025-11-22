use std::sync::atomic::{AtomicU32, Ordering};

use tilc_macros::uidx;
use tilc_span::{Ident, Span, Symbol};

use crate::{Delim, DelimSpan, Lit, TokenStream};

uidx! {
  pub struct NodeIdx {
    const DUMMY = Self::MAX.as_u32();
  }
}
pub const SANDYQ_NODE_IDX: NodeIdx = NodeIdx::from_u32(0);

uidx! {
  pub struct AttrIdx {}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Sandyq {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub items: Vec<Item>,

  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Attribute {
  pub idx: AttrIdx,

  pub path: Path,
  pub args: AttrArgs,

  pub style: AttributeStyle,
  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum AttributeStyle {
  Inner,
  Outer,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AttrArgs {
  Empty,
  Delimited(DelimArgs),
  Eq { eq_span: Span, expr: Box<Expr> },
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Item<K = ItemKind> {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Vis,
  pub kind: K,
  pub ident: Ident,

  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
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
#[derive(Debug)]
#[derive(Clone)]
pub enum ForeignItemKind {
  Static(Box<Static>),
  Fn(Box<Fn>),
}
#[derive(Debug)]
#[derive(Clone)]
pub enum AssociatedItemKind {
  Const(Box<Const>),
  Fn(Box<Fn>),
  TyAlias(Box<TyAlias>),
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Static {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Vis,
  pub mutability: Mutability,
  pub ident: Ident,
  pub ty: Box<Ty>,
  pub expr: Box<Expr>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Const {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Vis,
  pub ident: Ident,
  pub generics: Generics,
  pub ty: Box<Ty>,
  pub expr: Box<Expr>,

  pub defaultness: Defaultness,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Use {
  pub prefix: Box<Path>,
  pub kind: UseKind,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum UseKind {
  Single(Option<Ident>),
  Multiple(Vec<UseKind>),

  Everything,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Fn {
  pub generics: Generics,
  pub ident: Ident,
  pub fn_sig: FnSig,
  pub block: Option<Block>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FnSig {
  pub fn_header: FnHeader,
  pub fn_decl: FnDecl,
  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FnHeader {
  pub is_const: bool,
  pub is_async: bool,
  // TODO: Implement async and extern features
}
#[derive(Debug)]
#[derive(Clone)]
pub struct FnDecl {
  pub params: Vec<Param>,
  pub return_ty: FnReturnType,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum FnReturnType {
  Default,
  Other(Box<Ty>),
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Param {
  pub idx: NodeIdx,

  pub ty: Box<Ty>,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Generics {
  pub params: Vec<GenericParam>,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub struct GenericParam {
  pub idx: NodeIdx,
  pub ident: Ident,
  pub kind: GenericParamKind,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum GenericParamKind {
  Lifetime,
  Type,
  Const,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Block {
  pub idx: NodeIdx,

  pub statements: Vec<Statement>,
  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
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
  Expr(Box<Expr>),
  Semi(Box<Expr>),
}
impl StatementKind {
  fn access_attrs<F: FnMut(&mut Vec<Attribute>)>(&mut self, mut f: F) {
    match self {
      Self::Let(local) => f(&mut local.attrs),
      Self::Item(item) => f(&mut item.attrs),
      Self::Expr(expr) | Self::Semi(expr) => f(&mut expr.attrs),
    };
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Local {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub pat: Box<Pattern>,
  pub kind: LocalKind,
  pub ty: Option<Box<Ty>>,

  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum LocalKind {
  Decl,
  Init(Box<Expr>),
  InitElse(Box<Expr>, Box<Block>),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Pattern {
  pub idx: NodeIdx,

  pub kind: PatternKind,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum PatternKind {
  /// $mut? $ident $pat
  Ident(Mutability, Ident, Option<Box<Pattern>>),

  /// ($pat, $pat, ...)
  Tuple(Vec<Box<Pattern>>),

  /// $struct {$arg, $arg, ...}
  Struct(Path, Vec<FieldDef>),
  /// $expr
  Expr(Box<Expr>),
  /// &$mut? $pat
  Ref(Mutability, Box<Pattern>),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FieldDef {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Vis,
  pub ident: Option<Ident>,
  pub ty: Box<Ty>,

  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Korpe {
  Braced(Vec<Item>, Span),
  File,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ForeignKorpe {
  pub extern_span: Span,

  pub safety: Safety,
  pub abi: Option<Symbol>,
  pub items: Vec<Item<ForeignItemKind>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TyAlias {
  pub vis: Vis,
  pub ident: Ident,
  pub generics: Generics,
  pub ty: Option<Box<Ty>>,

  pub defaultness: Defaultness,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct EnumVariant {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub ident: Ident,

  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum VariantKind {
  Struct(Vec<FieldDef>),
  Tuple(Vec<FieldDef>),
  Unit,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Trait {
  pub idx: NodeIdx,

  pub vis: Vis,
  pub safety: Safety,
  pub ident: Ident,
  pub items: Vec<Item<AssociatedItemKind>>,

  pub span: Span,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Impl {
  pub safety: Safety,
  pub generics: Generics,
  pub kind: ImplKind,
  pub self_ty: Box<Ty>,
  pub items: Vec<Item<AssociatedItemKind>>,

  pub defaultness: Defaultness,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum ImplKind {
  Positive,
  Negative(Span),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct MacroCall {
  pub ident: Ident,
  pub args: DelimArgs,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct MacroDef {
  pub body: DelimArgs,
  pub is_macro_rules: bool,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Ty {
  pub idx: NodeIdx,

  pub kind: TyKind,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(Clone)]
pub enum Safety {
  Safe(Span),
  Unsafe(Span),
  Inherit,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Mutability {
  Mut,
  Nope,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Defaultness {
  Default,
  Overriden,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Vis {
  pub kind: VisKind,
  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum VisKind {
  Public,
  Private,
  Protected(NodeIdx, Box<Path>),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Path {
  pub segments: Vec<PathSegment>,
  pub span: Span,
}
#[derive(Debug, Clone)]
pub struct PathSegment {
  pub ident: Ident,
  pub idx: NodeIdx,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Expr {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub kind: ExprKind,

  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum ExprKind {
  /// ainymaly $pat = $expr
  Let(Box<Pattern>, Box<Expr>, Span),
  /// $lit
  Lit(Lit),
  /// $expr = $expr
  Assign(Box<Expr>, Span, Box<Expr>),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct DelimArgs {
  pub delim: Delim,
  pub span: DelimSpan,
  pub tokens: TokenStream,
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
