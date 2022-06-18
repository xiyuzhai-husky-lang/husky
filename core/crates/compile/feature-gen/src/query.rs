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
    fn main_feature_repr(&self, main_file: file::FilePtr) -> SemanticResult<FeatureRepr>;
    fn entity_feature_repr(&self, entity_route: EntityRoutePtr) -> SemanticResult<FeatureRepr>;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
    fn visual_feature_repr(&self, this: FeatureRepr) -> FeatureRepr;
}

fn main_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResult<FeatureRepr> {
    let pack = db.package(main_file)?;
    let main = &*pack.main_defn;
    Ok(FeatureRepr::from_defn(
        db,
        None,
        &main.defn_repr,
        db.feature_interner(),
    ))
}

fn entity_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> SemanticResult<FeatureRepr> {
    let entity = db.entity_defn(entity_route)?;
    match entity.variant {
        EntityDefnVariant::Feature { ref defn_repr, .. } => Ok(FeatureRepr::from_defn(
            db,
            None,
            defn_repr,
            db.feature_interner(),
        )),
        _ => todo!(),
    }
}
