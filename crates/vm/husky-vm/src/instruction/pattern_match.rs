use crate::*;

#[derive(Debug, PartialEq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMPattern>,
    pub body: Instructions,
}

#[derive(Debug, PartialEq)]
pub struct __PrimitiveValue;

#[derive(Debug, PartialEq)]
pub enum VMPattern {
    Primitive(__PrimitiveValue),
    EnumKind { kind_idx: i32 },
    Or(Vec<VMPattern>),
}

impl VMPattern {
    pub fn contains<'temp>(&self, value: &RegularValue) -> bool {
        match self {
            VMPattern::Primitive(primitive) => todo!(),
            // value == primitive,
            VMPattern::Or(subpatterns) => {
                for subpattern in subpatterns {
                    if subpattern.contains(value) {
                        return true;
                    }
                }
                false
            }
            VMPattern::EnumKind { kind_idx } => {
                todo!()
                // let value: &__VirtualEnum = value.downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
                // value.kind_idx == *kind_idx
            }
        }
    }
}
