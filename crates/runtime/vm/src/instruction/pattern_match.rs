use crate::*;
use entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq, Eq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMCasePattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VMCasePattern {
    Primitive(CopyableValue),
    OneOf(Vec<VMCasePattern>),
    EnumKindLiteral(EntityRoutePtr),
}

impl VMCasePattern {
    pub fn matches<'vm, 'eval>(&self, value: &TempValue<'vm, 'eval>) -> bool {
        match self {
            VMCasePattern::Primitive(v0) => match value {
                TempValue::Moved => todo!(),
                TempValue::Copyable(v1) => v0 == v1,
                TempValue::EvalOwned(_) => todo!(),
                TempValue::EvalPure(_) => todo!(),
                TempValue::EvalRef(_) => todo!(),
                TempValue::FullyOwnedRef(value) => todo!(),
                TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
                TempValue::TempOwned(_) => todo!(),
                TempValue::PartiallyOwnedRef(_) => todo!(),
                TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            },
            VMCasePattern::OneOf(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.matches(value) {
                        return true;
                    }
                }
                false
            }
            VMCasePattern::EnumKindLiteral(route) => match value {
                TempValue::Moved => todo!(),
                TempValue::Copyable(copyable_value) => match copyable_value {
                    CopyableValue::I32(_) => todo!(),
                    CopyableValue::F32(_) => todo!(),
                    CopyableValue::B32(_) => todo!(),
                    CopyableValue::B64(_) => todo!(),
                    CopyableValue::Bool(_) => todo!(),
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(enum_kind) => enum_kind.route == *route,
                },
                TempValue::EvalOwned(_) => todo!(),
                TempValue::EvalPure(_) => todo!(),
                TempValue::EvalRef(_) => todo!(),
                TempValue::FullyOwnedRef(value) => todo!(),
                TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
                TempValue::TempOwned(_) => todo!(),
                TempValue::PartiallyOwnedRef(_) => todo!(),
                TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            },
        }
    }
}
