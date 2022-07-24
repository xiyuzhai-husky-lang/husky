use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMCasePattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq)]
pub enum VMCasePattern {
    Primitive(PrimitiveValueData),
    OneOf(Vec<VMCasePattern>),
    EnumKindLiteral(EntityRoutePtr),
}

impl VMCasePattern {
    pub fn matches<'temp, 'eval>(&self, value: &__Register<'eval>) -> bool {
        match self {
            VMCasePattern::Primitive(v0) => {
                todo!()
                //     match value {
                //     __TempValue::Moved => todo!(),
                //     __TempValue::Copyable(v1) => v0 == v1,
                //     __TempValue::OwnedEval(_) => todo!(),
                //     __TempValue::EvalPure(_) => todo!(),
                //     __TempValue::EvalRef(_) => todo!(),
                //     __TempValue::TempRefEval(value) => todo!(),
                //     __TempValue::TempRefMutEval { value, owner, gen } => todo!(),
                //     __TempValue::OwnedTemp(_) => todo!(),
                //     __TempValue::TempRefTemp(_) => todo!(),
                //     __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
                // }
            }
            VMCasePattern::OneOf(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.matches(value) {
                        return true;
                    }
                }
                false
            }
            VMCasePattern::EnumKindLiteral(route) => {
                todo!()
                //     match value {
                //     __TempValue::Moved => todo!(),
                //     __TempValue::Copyable(copyable_value) => match copyable_value {
                //         PrimitiveValueData::EnumKind(enum_kind) => enum_kind.route == *route,
                //         _ => todo!(),
                //     },
                //     __TempValue::OwnedEval(_) => todo!(),
                //     __TempValue::EvalPure(_) => todo!(),
                //     __TempValue::EvalRef(_) => todo!(),
                //     __TempValue::TempRefEval(value) => todo!(),
                //     __TempValue::TempRefMutEval { value, owner, gen } => todo!(),
                //     __TempValue::OwnedTemp(_) => todo!(),
                //     __TempValue::TempRefTemp(_) => todo!(),
                //     __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
                // },
            }
        }
    }
}
