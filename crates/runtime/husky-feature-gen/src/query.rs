mod entity_feature_repr;
mod main_feature_repr;

pub use entity_feature_repr::*;
use husky_data_viewer::HuskyDataViewerQueryGroup;
use husky_trace_protocol::Restriction;
pub use main_feature_repr::*;
use vm::{InterpreterQueryGroup, __ModelLinkage, __Register, __VMResult};

use crate::{intern::InternFeature, record::*, visual::*, *};
use husky_compile_time::AskCompileTime;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{EntityDefnQueryGroup, EntityDefnVariant};
use husky_instruction_gen::InstructionGenQueryGroup;
use husky_linkage_table::ResolveLinkage;
use husky_package_semantics::*;
use semantics_error::SemanticResult;
use std::{
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
};
use upcast::Upcast;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait FeatureGenQueryGroup:
    InternFeature
    + Upcast<dyn InstructionGenQueryGroup>
    + InstructionGenQueryGroup
    + Upcast<dyn InterpreterQueryGroup>
    + HuskyDataViewerQueryGroup
    + Upcast<dyn HuskyDataViewerQueryGroup>
    + TrainModel
    + RefUnwindSafe
    + UnwindSafe
{
    fn main_feature_repr(&'eval self, target_entrance: husky_file::FilePtr) -> FeatureRepr;
    fn entity_feature_repr(&self, entity_route: EntityRoutePtr) -> FeatureRepr;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
    fn visual_feature_lazy_block(&self, this: FeatureRepr) -> __VMResult<Arc<FeatureLazyBlock>>;
}

pub trait TrainModel {
    fn train(
        &self,
        model: __ModelLinkage,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        opds: &[Arc<FeatureLazyExpr>],
    ) -> __VMResult<__Register<'static>>;
}
