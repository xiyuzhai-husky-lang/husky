use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::PureBinaryOpr;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyOpnKind {
    Binary { opr: PureBinaryOpr, this: ScopePtr },
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
