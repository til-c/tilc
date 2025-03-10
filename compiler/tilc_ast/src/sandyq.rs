use tilc_index::uidx;
use tilc_span::{Identifier, Span};


uidx! {
  #[derive(Debug)]
  pub struct AttributeIdx;

  #[derive(Debug)]
  pub struct NodeIdx;
}


#[derive(Debug)]
pub struct Visibility {
  pub kind: VisibilityKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum VisibilityKind {
  Public,
  Private,

  // TODO:
  /// Sandyq specific (configurable)
  ///
  /// barsh(sandyq::alma ishinde)
  Protected(Box<Path>),
}
#[derive(Debug)]
pub struct Attribute {
  pub idx: AttributeIdx,
  pub span: Span,
}
#[derive(Debug)]
pub struct Item<K = ItemKind> {
  pub idx: NodeIdx,

  pub ident: Identifier,
  pub span: Span,
  pub visibility: Visibility,

  pub kind: K,
}
pub type ItemInfo = (Identifier, ItemKind);

#[derive(Debug)]
pub struct Path {
  pub segments: Vec<PathSegment>,
  pub span: Span,
}
#[derive(Debug)]
pub struct PathSegment {
  pub ident: Identifier,
  pub idx: NodeIdx,
}


#[derive(Debug)]
pub struct Const {
  pub idx: NodeIdx,
  pub expression: Expression,
}
#[derive(Debug)]
pub struct MutTy {
  pub ty: Box<Ty>,
  pub mutable: bool,
}
#[derive(Debug)]
pub struct Lifetime {
  pub idx: NodeIdx,
  pub ident: Identifier,
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
  Ref(Option<Box<Lifetime>>, Box<MutTy>),

  // TODO:
  /// Pointer type
  Ptr(),

  /// Fixed array size
  ///
  /// [b8; 2]
  Array(Box<Ty>, Box<Const>),

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
pub struct Ty {
  pub idx: NodeIdx,

  pub kind: TyKind,
  pub span: Span,
}

#[derive(Debug)]
pub enum GenericParamKind {
  Lifetime,
  Type,
  Const,
}

#[derive(Debug)]
pub struct GenericParam {
  pub idx: NodeIdx,
  pub ident: Identifier,
  pub kind: GenericParamKind,
}

#[derive(Debug)]
pub struct Generics {
  pub params: Vec<GenericParam>,
  pub span: Span,
}

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
pub enum ExpressionKind {}
#[derive(Debug)]
pub struct Expression {
  pub idx: NodeIdx,
  pub kind: ExpressionKind,
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
pub struct Statement {
  pub idx: NodeIdx,
  pub kind: StatementKind,
  pub span: Span,
}
#[derive(Debug)]
pub struct Block {
  pub idx: NodeIdx,

  pub statements: Vec<Statement>,
  pub span: Span,
}


#[derive(Debug)]
pub struct Param {
  pub idx: NodeIdx,

  pub ty: Box<Ty>,
  pub span: Span,
}
#[derive(Debug)]
pub enum FnReturnType {
  Default,
  Other(Box<Ty>),
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
pub struct FnSig {
  pub fn_header: FnHeader,
  pub fn_decl: FnDecl,
  pub span: Span,
}
#[derive(Debug)]
pub struct Fn {
  pub generics: Generics,
  pub fn_sig: FnSig,
  pub block: Option<Block>,
}
#[derive(Debug)]
pub enum UseKind {
  Single(Option<Identifier>),
  Multiple(Vec<UseKind>),

  Everything,
}
#[derive(Debug)]
pub struct Use {
  pub prefix: Box<Path>,
  pub kind: UseKind,
  pub span: Span,
}
#[derive(Debug)]
pub enum ItemKind {
  Fn(Box<Fn>),

  Use(Box<Use>),

  Const(),
  Static(),

  TyAlias(),

  Struct(),
  Enum(),
  Union(),

  Mod(),

  Trait(),
  Impl(),
}


/// Package struct
#[derive(Debug)]
pub struct Sandyq {
  pub idx: NodeIdx,

  pub attributes: Vec<Attribute>,

  pub items: Vec<Item>,
}
