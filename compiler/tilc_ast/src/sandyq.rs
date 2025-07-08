use tilc_macro::uidx;
use tilc_span::{Ident, Span, Symbol};

use crate::{DelimSpacing, DelimSpan, TokenStream};


uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
  pub struct NodeIdx {}
}
pub const SANDYQ_NODE_IDX: NodeIdx = NodeIdx::from_u32(0);

uidx! {
  #[derive(Debug, Clone, Copy)]
  pub struct AttrIdx {}
}


#[derive(Debug)]
pub struct Sandyq {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub items: Vec<Item>,

  pub span: Span,
}

#[derive(Debug)]
pub struct Attribute {
  pub idx: AttrIdx,

  pub path: Path,
  pub args: AttrArgs,

  pub style: AttrkibuteStyle,
  pub span: Span,
}
#[derive(Debug)]
pub enum AttrkibuteStyle {
  Inner,
  Outer,
}
#[derive(Debug)]
pub enum AttrArgs {
  Empty,
  Delimited(DelimArgs),
  Eq {
    eq_span: Span,
    expr: Box<Expression>,
  },
}

#[derive(Debug)]
pub struct Item<K = ItemKind> {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub kind: K,
  pub ident: Ident,

  pub span: Span,
}

#[derive(Debug)]
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
pub enum ForeignItemKind {
  Static(Box<Static>),
  Fn(Box<Fn>),
}
#[derive(Debug)]
pub enum AssociatedItemKind {
  Const(Box<Const>),
  Fn(Box<Fn>),
  TyAlias(Box<TyAlias>),
}

#[derive(Debug)]
pub struct MacroDef {
  pub body: DelimArgs,
  pub is_macro_rules: bool,
}
#[derive(Debug)]
pub struct MacroCall {
  pub ident: Ident,
  pub args: DelimArgs,
}
#[derive(Debug)]
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
pub enum ImplKind {
  Positive,
  Negative(Span),
}

#[derive(Debug)]
pub struct Trait {
  pub idx: NodeIdx,

  pub vis: Visibility,
  pub safety: Safety,
  pub ident: Ident,
  pub items: Vec<Item<AssociatedItemKind>>,

  pub span: Span,
}

#[derive(Debug)]
pub struct EnumVariant {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub ident: Ident,

  pub span: Span,
}
#[derive(Debug)]
pub enum VariantKind {
  Struct(Vec<FieldDef>),
  Tuple(Vec<FieldDef>),
  Unit,
}

#[derive(Debug)]
pub struct FieldDef {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub ident: Option<Ident>,
  pub ty: Box<Ty>,

  pub span: Span,
}
#[derive(Debug)]
pub struct TyAlias {
  pub vis: Visibility,
  pub ident: Ident,
  pub generics: Generics,
  pub ty: Option<Box<Ty>>,

  pub defaultness: Defaultness,
}
#[derive(Debug)]
pub struct Static {
  pub idx: NodeIdx,

  pub attrs: Vec<Attribute>,
  pub vis: Visibility,
  pub mutability: Mutability,
  pub ident: Ident,
  pub ty: Box<Ty>,
  pub expr: Box<Expression>,
}

#[derive(Debug)]
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


#[derive(Debug)]
pub struct ForeignKorpe {
  pub extern_span: Span,

  pub safety: Safety,
  pub abi: Option<Symbol>,
  pub items: Vec<Item<ForeignItemKind>>,
}
#[derive(Debug)]
pub enum Korpe {
  Braced(Vec<Item>, Span),
  File,
}
#[derive(Debug)]
pub struct Path {
  pub segments: Vec<PathSegment>,
  pub span: Span,
}
#[derive(Debug)]
pub struct PathSegment {
  pub ident: Ident,
  pub idx: NodeIdx,
}

#[derive(Debug)]
pub struct Fn {
  pub generics: Generics,
  pub fn_sig: FnSig,
  pub block: Option<Block>,
}
#[derive(Debug)]
pub struct FnSig {
  pub fn_header: FnHeader,
  pub fn_decl: FnDecl,
  pub span: Span,
}
#[derive(Debug)]
pub struct FnHeader {
  pub is_const: bool,
  pub is_async: bool,
  // TODO: Implement async and extern features
}
#[derive(Debug)]
pub struct FnDecl {
  pub params: Vec<Param>,
  pub return_ty: FnReturnType,
}
#[derive(Debug)]
pub enum FnReturnType {
  Default,
  Other(Box<Ty>),
}
#[derive(Debug)]
pub struct Param {
  pub idx: NodeIdx,

  pub ty: Box<Ty>,
  pub span: Span,
}
#[derive(Debug)]
pub struct Generics {
  pub params: Vec<GenericParam>,
  pub span: Span,
}
#[derive(Debug)]
pub struct GenericParam {
  pub idx: NodeIdx,
  pub ident: Ident,
  pub kind: GenericParamKind,
}
#[derive(Debug)]
pub enum GenericParamKind {
  Lifetime,
  Type,
  Const,
}


#[derive(Debug)]
pub struct Ty {
  pub idx: NodeIdx,

  pub kind: TyKind,
  pub span: Span,
}
#[derive(Debug)]
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
pub struct Block {
  pub idx: NodeIdx,

  pub statements: Vec<Statement>,
  pub span: Span,
}
#[derive(Debug)]
pub struct Statement {
  pub idx: NodeIdx,
  pub kind: StatementKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum StatementKind {
  Let(Box<Local>),
  Item(Box<Item>),
  Expression(Box<Expression>),
  Semi(Box<Expression>),
}
#[derive(Debug)]
pub struct Expression {
  pub idx: NodeIdx,
  pub kind: ExpressionKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum ExpressionKind {}
#[derive(Debug)]
pub struct Local {
  pub idx: NodeIdx,

  pub kind: LocalKind,
  pub ty: Option<Ty>,
  pub span: Span,
}
#[derive(Debug)]
pub enum LocalKind {
  Decl,
  Init(Expression),
}

#[derive(Debug)]
pub struct Use {
  pub prefix: Box<Path>,
  pub kind: UseKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum UseKind {
  Single(Option<Ident>),
  Multiple(Vec<UseKind>),

  Everything,
}

#[derive(Debug)]
pub struct Visibility {
  pub kind: VisibilityKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum VisibilityKind {
  Public,
  Private(Box<Path>),
  Protected,
}

#[derive(Debug)]
pub struct DelimArgs {
  pub spacing: DelimSpacing,
  pub span: DelimSpan,
  pub tokens: TokenStream,
}


#[derive(Debug)]
pub enum Safety {
  Safe(Span),
  Unsafe(Span),
  Inherit,
}
#[derive(Debug)]
pub enum Mutability {
  Mut,
  Nope,
}
#[derive(Debug)]
pub enum Defaultness {
  Default,
  Overriden,
}
