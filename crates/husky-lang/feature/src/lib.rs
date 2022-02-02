mod cache;
mod eval;
mod intern;
mod query;

use common::*;
use vm::{BinaryOpr, PrimitiveValue};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    Literal(PrimitiveValue),
    Assert {
        condition: FeatureId,
    },
    Do {
        first: FeatureId,
        then: FeatureId,
    },
    PrimitiveBinaryFunc {
        func: BinaryOpr,
        lopd: FeatureId,
        ropd: FeatureId,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FeatureId {
    pub(crate) raw: usize,
    pub(crate) cached: bool,
}
