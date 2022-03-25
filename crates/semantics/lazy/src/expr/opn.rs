use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use vm::PureBinaryOpr;
use word::CustomIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyOpnKind {
    Binary { opr: PureBinaryOpr, this: ScopePtr },
    Prefix(PrefixOpr),
    RoutineCall(RangedScope),
    TypeCall(RangedScope),
    PatternCall,
    MembAccess(CustomIdentifier),
    MembCall { memb_ident: CustomIdentifier },
    ElementAccess,
}
