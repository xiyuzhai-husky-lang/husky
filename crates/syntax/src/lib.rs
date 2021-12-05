#![allow(dead_code, unused)]
use std::{marker::PhantomData, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult<T> {
    _ty: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleFileParseTree {}

pub use smol_str::SmolStr;

pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};

pub mod ast {
    pub enum FormatSpecifier {}
    pub enum HasFormatSpecifier {}
    pub struct TryExpr {}
    pub struct PrefixExpr {}
    pub struct IfExpr {}
    pub struct WhereClause {}
    pub struct RecordFieldList {}
    pub struct Enum {}
    pub struct GenericParamList {}
    pub struct Module {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Item {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SelfParam {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Name {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Pat {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NameRef {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Type {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Impl {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct GenericArgList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Path {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NameLike {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SingleFileParseTree {}
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SyntaxNodePtr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Label {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct StmtList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BreakExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Expr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Attr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Ident {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PathSegmentKind {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ItemList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MethodCallExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct IdentPat {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Use {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UseTree {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PathSegment {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Visibility {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VisibilityKind {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BlockExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RecordExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Const {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RecordPat {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Param {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Fn {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TypeAlias {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CallExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Comment {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SelfParamKind {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RecordExprField {}
    pub struct TokenTree {}
    pub struct AssocItem {}
    pub struct String {}
    pub struct Adt {}
    pub struct ParamList {}
    pub struct ArgList {}
    pub struct AssocItemList {}
    pub struct LetStmt {}
    pub struct ImplTraitType {}
    pub struct MatchArm {}
    pub struct MatchArmList {}
    pub struct MatchExpr {}
    pub struct RetType {}
    pub mod edit {
        pub struct IndentLevel {}
    }
    pub mod edit_in_place {
        pub struct Indent {}
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TupleField {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Struct {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Variant {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TupleFieldList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FieldExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UseTreeList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct GenericParam {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ExprStmt {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BinaryOp {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BinExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CallableExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PathExpr {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FieldList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct VariantList {}
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NameOrNameRef {}
}
pub struct SyntaxElementChildren {}
pub struct SyntaxToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxKind {}
pub struct SyntaxElement {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode {}
pub struct SyntaxNodePtr {}
