use scope::ScopePtr;
use syntax_types::*;
use text::TextRange;
use vm::BinaryOpr;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opn {
    Binary {
        opr: BinaryOpr,
        this: ScopePtr,
        kind: BinaryOpnKind,
    },
    Prefix(PrefixOpn),
    Suffix(SuffixOpn),
    FuncCall {
        func: ScopePtr,
    },
    PattCall,
    MembVarAccess,
    MembFuncCall(ScopePtr),
    ElementAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    EqBool,
    // 32 binary bits
    BitAnd32,
    BitOr32,
    // custom
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefixOpn {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuffixOpn {}
