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
    pub fn matches<'vm, 'eval>(&self, value: &VMValue<'vm, 'eval>) -> bool {
        match self {
            VMCasePattern::Primitive(v0) => match value {
                VMValue::Moved => todo!(),
                VMValue::Copyable(v1) => v0 == v1,
                VMValue::FullyOwned(_) => todo!(),
                VMValue::EvalPure(_) => todo!(),
                VMValue::EvalRef(_) => todo!(),
                VMValue::FullyOwnedRef(value) => todo!(),
                VMValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
                VMValue::PartiallyOwned(_) => todo!(),
                VMValue::PartiallyOwnedRef(_) => todo!(),
                VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
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
                VMValue::Moved => todo!(),
                VMValue::Copyable(copyable_value) => match copyable_value {
                    CopyableValue::I32(_) => todo!(),
                    CopyableValue::F32(_) => todo!(),
                    CopyableValue::B32(_) => todo!(),
                    CopyableValue::B64(_) => todo!(),
                    CopyableValue::Bool(_) => todo!(),
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(enum_kind) => enum_kind.route == *route,
                },
                VMValue::FullyOwned(_) => todo!(),
                VMValue::EvalPure(_) => todo!(),
                VMValue::EvalRef(_) => todo!(),
                VMValue::FullyOwnedRef(value) => todo!(),
                VMValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
                VMValue::PartiallyOwned(_) => todo!(),
                VMValue::PartiallyOwnedRef(_) => todo!(),
                VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            },
        }
    }
}
