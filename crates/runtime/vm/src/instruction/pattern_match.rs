use crate::*;
use entity_route::EntityRoutePtr;

#[derive(Debug, PartialEq, Eq)]
pub struct VMPatternBranch {
    pub opt_pattern: Option<VMCasePattern>,
    pub body: Arc<InstructionSheet>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VMCasePattern {
    Primitive(PrimitiveValue),
    OneOf(Vec<VMCasePattern>),
    EnumLiteral(EntityRoutePtr),
}
