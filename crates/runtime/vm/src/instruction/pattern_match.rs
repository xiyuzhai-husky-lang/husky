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
    pub fn matches<'stack, 'eval>(&self, value: &StackValue<'stack, 'eval>) -> bool {
        match self {
            VMCasePattern::Primitive(v0) => match value {
                StackValue::Moved => todo!(),
                StackValue::Copyable(v1) => v0 == v1,
                StackValue::Owned(_) => todo!(),
                StackValue::GlobalPure(_) => todo!(),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef { value, owner, gen } => todo!(),
                StackValue::LocalRefMut { value, owner, gen } => todo!(),
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
                StackValue::Moved => todo!(),
                StackValue::Copyable(copyable_value) => match copyable_value {
                    CopyableValue::I32(_) => todo!(),
                    CopyableValue::F32(_) => todo!(),
                    CopyableValue::B32(_) => todo!(),
                    CopyableValue::B64(_) => todo!(),
                    CopyableValue::Bool(_) => todo!(),
                    CopyableValue::Void => todo!(),
                    CopyableValue::EnumKind(enum_kind) => enum_kind.route == *route,
                },
                StackValue::Owned(_) => todo!(),
                StackValue::GlobalPure(_) => todo!(),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef { value, owner, gen } => todo!(),
                StackValue::LocalRefMut { value, owner, gen } => todo!(),
            },
        }
    }
}
