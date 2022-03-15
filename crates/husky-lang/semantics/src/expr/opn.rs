use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::BinaryOpr;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpnKind {
    Binary { opr: BinaryOpr, this: ScopePtr },
    Prefix(PrefixOpn),
    Suffix(SuffixOpn),
    RoutineCall(RangedScope),
    PattCall,
    MembVarAccess,
    MembFuncCall(ScopePtr),
    ElementAccess,
}

impl std::hash::Hash for OpnKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            OpnKind::Binary { opr, this } => todo!(),
            OpnKind::Prefix(_) => todo!(),
            OpnKind::Suffix(_) => todo!(),
            OpnKind::RoutineCall(_) => todo!(),
            OpnKind::PattCall => todo!(),
            OpnKind::MembVarAccess => todo!(),
            OpnKind::MembFuncCall(_) => todo!(),
            OpnKind::ElementAccess => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefixOpn {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuffixOpn {}
