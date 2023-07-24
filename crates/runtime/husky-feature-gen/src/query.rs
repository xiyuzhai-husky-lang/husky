mod item_feature_repr;
mod main_feature_repr;

use crate::{intern::InternFeature, record::*, visual::*, *};
use husky_data_viewer::DataViewerDb;
use husky_ethereal_term::EtherealTerm;
use husky_instruction_gen::InstructionDb;
use husky_item_semantics::{EntityDefnQueryGroup, EntityDefnVariant};
use husky_package_semantics::*;
use husky_vm::{InterpreterQueryGroup, __ModelLinkage, __Register, __VMResult};
use item_feature_repr::item_feature_repr;
use main_feature_repr::*;
use std::panic::{RefUnwindSafe, UnwindSafe};
use upcast::Upcast;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait FeatureGenQueryGroup:
    InternFeature
    + Upcast<dyn InstructionDb>
    + InstructionDb
    + Upcast<dyn InterpreterQueryGroup>
    + DataViewerDb
    + Upcast<dyn DataViewerDb>
    + TrainModel
    + RefUnwindSafe
    + UnwindSafe
{
    fn main_feature_repr(&'eval self, target_entrance: EntityPath) -> FeatureRepr;
    fn item_feature_repr(&self, item_path: EtherealTerm) -> FeatureRepr;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: Ident) -> FeatureRepr;
    fn visual_feature_lazy_block(&self, this: FeatureRepr) -> __VMResult<Arc<FeatureLazyBody>>;
}

pub trait TrainModel {
    fn train(
        &self,
        model: __ModelLinkage,
        opt_arrival_indicator: Option<&Arc<FeatureDomainIndicator>>,
        opds: &[Arc<FeatureLazyExpr>],
    ) -> __VMResult<__Register<'static>>;
}
