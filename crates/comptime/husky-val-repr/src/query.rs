mod item_feature_repr;
mod main_feature_repr;

use crate::{intern::InternFeature, record::*, visual::*, *};
use husky_data_viewer::DataViewerDb;
use husky_ethereal_term::EtherealTerm;
use husky_instruction_gen::InstructionDb;
use husky_item_semantics::{EntityDefnQueryGroup, EntityDefnVariant};
use husky_package_semantics::*;
use husky_vm::{InterpreterQueryGroup, __ModelLinkageGroup, __RegularValue, __VMResult};
use item_feature_repr::item_feature_repr;
use main_feature_repr::*;
use std::panic::{RefUnwindSafe, UnwindSafe};
use upcast::Upcast;

#[salsa::query_group(FeatureGenQueryGroupStorage)]
pub trait ValReprDb:
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
    fn main_feature_repr(&'static self, target_entrance: EntityPath) -> ValRepr;
    fn item_feature_repr(&self, item_path: EtherealTerm) -> ValRepr;
    fn record_field_repr(&self, this: ValRepr, field_ident: Ident) -> ValRepr;
    fn visual_feature_lazy_block(&self, this: ValRepr) -> __VMResult<ValBlock>;
}

pub trait TrainModel {
    fn train(
        &self,
        model: __ModelLinkageGroup,
        opt_arrival_indicator: Option<&ValDomain>,
        opds: &[ValExpr],
    ) -> __VMResult<__RegularValue>;
}
