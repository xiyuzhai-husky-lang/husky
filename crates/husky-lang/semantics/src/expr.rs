use scope::ScopeId;
use syntax_types::*;
use text::TextRange;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub range: TextRange,
    pub ty: ScopeId,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Variable(Identifier),
    Scope {
        id: ScopeId,
        compiled: Option<()>,
    },
    Literal(Literal),
    Bracketed(Box<Expr>),
    Opn {
        opn: Opn,
        compiled: Option<()>,
        opds: Vec<Expr>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopeId>)>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opn {
    Binary {
        opr: BinaryOpr,
        this: ScopeId,
        kind: BinaryOpnKind,
    },
    Prefix(PrefixOpn),
    Suffix(SuffixOpn),
    FuncCall,
    PattCall,
    MembVarAccess,
    MembFuncCall(ScopeId),
    ElementAccess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOpnKind {
    // ordered field i32
    AddI32,
    SubI32,
    MulI32,
    DivI32,
    EqI32,
    NeqI32,
    GreaterI32,
    GeqI32,
    LessI32,
    LeqI32,
    // ordered field f32
    AddF32,
    SubF32,
    MulF32,
    DivF32,
    EqF32,
    NeqF32,
    GreaterF32,
    GeqF32,
    LessF32,
    LeqF32,
    // bool
    And,
    Or,
    // 32 binary bits
    BitAnd32,
    BitOr32,
    // custom
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOpn {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuffixOpn {}
