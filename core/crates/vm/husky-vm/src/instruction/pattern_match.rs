use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMPattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq)]
pub enum VMPattern {
    Primitive(PrimitiveValueData),
    OneOf(Vec<VMPattern>),
    EnumKindLiteral(EntityRoutePtr),
}

impl VMPattern {
    pub fn matches<'temp, 'eval>(&self, value: &__Register<'eval>) -> bool {
        match self {
            VMPattern::Primitive(primitive) => value.match_primitive(*primitive),
            VMPattern::OneOf(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.matches(value) {
                        return true;
                    }
                }
                false
            }
            VMPattern::EnumKindLiteral(route) => {
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
