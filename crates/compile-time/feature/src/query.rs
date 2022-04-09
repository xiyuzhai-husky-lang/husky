use entity_route::EntityRoutePtr;
use instruction_gen::InstructionGenQueryGroup;
use pack_semantics::*;
use semantics_entity::{EntityDefnVariant, EntityQueryGroup};
use semantics_error::SemanticResultArc;
use upcast::Upcast;
use vm::InterpreterQueryGroup;

use crate::{record::*, unique_allocate::AllocateUniqueFeature, *};

#[salsa::query_group(FeatureQueryGroupStorage)]
pub trait FeatureQueryGroup:
    AllocateUniqueFeature
    + PackQueryGroup
    + Upcast<dyn EntityQueryGroup>
    + InstructionGenQueryGroup
    + Upcast<dyn InterpreterQueryGroup>
{
    fn main_feature_block(&self, main_file: file::FilePtr) -> SemanticResultArc<FeatureBlock>;
    fn scoped_feature_block(&self, scope: EntityRoutePtr) -> SemanticResultArc<FeatureBlock>;
    fn record_field_repr(&self, this: FeatureRepr, field_ident: CustomIdentifier) -> FeatureRepr;
}

fn main_feature_block(
    db: &dyn FeatureQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<FeatureBlock> {
    let pack = db.package(main_file)?;
    let main = &*pack.main_defn;
    Ok(FeatureBlock::new(db, None, &main.stmts, &[], db.features()))
}

fn scoped_feature_block(
    db: &dyn FeatureQueryGroup,
    scope: EntityRoutePtr,
) -> SemanticResultArc<FeatureBlock> {
    let entity = db.opt_entity_defn(scope)?.unwrap();
    match entity.kind() {
        EntityDefnVariant::Feature { ref lazy_stmts, .. } => {
            Ok(FeatureBlock::new(db, None, lazy_stmts, &[], db.features()))
        }
        _ => todo!(),
    }
}
