mod entity_feature_repr;
mod main_feature_repr;

pub use entity_feature_repr::*;
pub use main_feature_repr::*;
use vm::EvalResult;

use crate::{record::*, unique_allocate::AllocateUniqueFeature, visual::*, *};
use entity_route::EntityRoutePtr;
use husky_compile_time::AskCompileTime;
use instruction_gen::InstructionGenQueryGroup;
use linkage_table::ResolveLinkage;
use pack_semantics::*;
use semantics_entity::{EntityDefnQueryGroup, EntityDefnVariant};
use semantics_error::SemanticResult;
use std::marker::PhantomData;
use upcast::Upcast;
use visualizer_gen::VisualizerQueryGroup;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait FeatureGenQueryGroup:
    AllocateUniqueFeature
    + Upcast<dyn InstructionGenQueryGroup>
    + InstructionGenQueryGroup
    + AskCompileTime
    + VisualizerQueryGroup
{
    fn main_feature_repr(&'eval self, main_file: file::FilePtr) -> FeatureRepr;
    fn entity_feature_repr(&self, entity_route: EntityRoutePtr) -> FeatureRepr;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
    fn visual_feature_repr(&self, this: FeatureRepr) -> EvalResult<FeatureRepr>;
}
