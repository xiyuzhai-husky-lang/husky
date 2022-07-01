use crate::*;
use husky_entity_route_syntax::EntityRoutePtr;

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
    pub fn matches<'temp, 'eval>(&self, value: &TempValue<'temp, 'eval>) -> bool {
        match self {
            VMCasePattern::Primitive(v0) => match value {
                TempValue::Moved => todo!(),
                TempValue::Copyable(v1) => v0 == v1,
                TempValue::OwnedEval(_) => todo!(),
                TempValue::EvalPure(_) => todo!(),
                TempValue::EvalRef(_) => todo!(),
                TempValue::TempRefEval(value) => todo!(),
                TempValue::TempRefMutEval { value, owner, gen } => todo!(),
                TempValue::OwnedTemp(_) => todo!(),
                TempValue::TempRefTemp(_) => todo!(),
                TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
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
                    CopyableValue::EnumKind(enum_kind) => enum_kind.route == *route,
                    _ => todo!(),
                },
                TempValue::OwnedEval(_) => todo!(),
                TempValue::EvalPure(_) => todo!(),
                TempValue::EvalRef(_) => todo!(),
                TempValue::TempRefEval(value) => todo!(),
                TempValue::TempRefMutEval { value, owner, gen } => todo!(),
                TempValue::OwnedTemp(_) => todo!(),
                TempValue::TempRefTemp(_) => todo!(),
                TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            },
        }
    }
}
