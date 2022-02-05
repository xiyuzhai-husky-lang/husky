mod block;
mod eval;
mod expr;
mod query;
mod sheet;
mod stmt;
mod unique_allocate;

pub use query::{FeatureQueryGroup, FeatureQueryGroupStorage};
pub use unique_allocate::{
    new_feature_unique_allocator, AllocateUniqueFeature, FeatureUniqueAllocator,
};

use common::*;
use unique_allocate::FeaturePtr;
use vm::{BinaryOpr, PrimitiveValue};
use word::CustomIdentifier;

use block::FeatureBlock;
use expr::FeatureExpr;
use stmt::FeatureStmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSymbol {
    varname: CustomIdentifier,
    feature: FeaturePtr,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    Literal(PrimitiveValue),
    Assert {
        condition: FeaturePtr,
    },
    Block(Vec<FeaturePtr>),
    PrimitiveBinaryOpr {
        opr: BinaryOpr,
        lopd: FeaturePtr,
        ropd: FeaturePtr,
    },
}

impl From<&Feature> for Feature {
    fn from(feature: &Feature) -> Self {
        feature.clone()
    }
}
