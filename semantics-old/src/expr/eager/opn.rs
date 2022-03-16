use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::BinaryOpr;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpnKind {
    Binary { opr: BinaryOpr, this: ScopePtr },
    Prefix(PrefixOpn),
    Suffix(SuffixOpn),
    RoutineCall(RangedScope),
    TypeCall(RangedScope),
    PatternCall,
    MembVarAccess,
    MembFuncCall(ScopePtr),
    ElementAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefixOpn {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuffixOpn {}
