mod entity_feature_repr;
mod main_feature_repr;

pub use entity_feature_repr::*;
pub use main_feature_repr::*;
use vm::{InterpreterQueryGroup, ModelLinkage, __EvalResult};

use crate::{record::*, unique_allocate::AllocateUniqueFeature, visual::*, *};
use husky_compile_time::AskCompileTime;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{EntityDefnQueryGroup, EntityDefnVariant};
use husky_instruction_gen::InstructionGenQueryGroup;
use husky_linkage_table::ResolveLinkage;
use husky_package_semantics::*;
use semantics_error::SemanticResult;
use std::marker::PhantomData;
use upcast::Upcast;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait FeatureGenQueryGroup:
    AllocateUniqueFeature
    + Upcast<dyn InstructionGenQueryGroup>
    + InstructionGenQueryGroup
    + Upcast<dyn InterpreterQueryGroup>
    + AskCompileTime
    + TrainModel
{
    fn main_feature_repr(&'eval self, main_file: husky_file::FilePtr) -> FeatureRepr;
    fn entity_feature_repr(&self, entity_route: EntityRoutePtr) -> FeatureRepr;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
    fn visual_feature_repr(&self, this: FeatureRepr) -> __EvalResult<FeatureRepr>;
}

pub trait TrainModel {
    fn train(
        &self,
        model: ModelLinkage,
        opt_branch_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        opds: &[Arc<FeatureExpr>],
    ) -> __EvalResult;
}
