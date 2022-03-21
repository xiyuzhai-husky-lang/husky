use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::PureBinaryOpr;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyOpnKind {
    Binary { opr: PureBinaryOpr, this: ScopePtr },
    Prefix(PrefixOpr),
    RoutineCall(RangedScope),
    TypeCall(RangedScope),
    PatternCall,
    MembVarAccess(CustomIdentifier),
    MembFuncCall(ScopePtr),
    ElementAccess,
}
