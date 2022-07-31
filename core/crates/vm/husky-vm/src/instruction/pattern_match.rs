use crate::*;
use husky_entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMPattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq)]
pub enum VMPattern {
    Primitive(__Register<'static>),
    OneOf(Vec<VMPattern>),
    EnumKindLiteral(EntityRoutePtr),
}

impl VMPattern {
    pub fn matches<'temp, 'eval>(&self, value: &__Register<'eval>) -> bool {
        match self {
            VMPattern::Primitive(primitive) => value == primitive,
            VMPattern::OneOf(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.matches(value) {
                        return true;
                    }
                }
                false
            }
            VMPattern::EnumKindLiteral(route) => {
                let value: &VirtualEnum = unsafe { value.downcast_temp_ref() };
                value.route == *route
            }
        }
    }
}
