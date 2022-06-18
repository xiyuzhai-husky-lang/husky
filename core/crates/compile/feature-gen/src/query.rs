mod entity_feature_repr;
mod main_feature_repr;

use std::marker::PhantomData;

pub use entity_feature_repr::*;
pub use main_feature_repr::*;

use crate::{record::*, unique_allocate::AllocateUniqueFeature, visual::*, *};
use entity_route::EntityRoutePtr;
use instruction_gen::InstructionGenQueryGroup;
use linkage_table::ResolveLinkage;
use pack_semantics::*;
use semantics_entity::{EntityDefnQueryGroup, EntityDefnVariant};
use semantics_error::SemanticResult;
use upcast::Upcast;
use visualizer_gen::VisualizerQueryGroup;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait FeatureGenQueryGroup:
    AllocateUniqueFeature
    + PackageQueryGroup
    + Upcast<dyn EntityDefnQueryGroup>
    + Upcast<dyn InstructionGenQueryGroup>
    + InstructionGenQueryGroup
    + ResolveLinkage
    + VisualizerQueryGroup
{
    fn main_feature_repr(&'eval self, main_file: file::FilePtr) -> SemanticResult<FeatureRepr>;
    fn entity_feature_repr(&self, entity_route: EntityRoutePtr) -> SemanticResult<FeatureRepr>;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
    fn visual_feature_repr(&self, this: FeatureRepr) -> FeatureRepr;
}
