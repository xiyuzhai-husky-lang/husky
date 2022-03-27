use scope::ScopePtr;
use semantics_entity::{EntityKind, EntityQueryGroup};
use semantics_error::SemanticResultArc;
use semantics_package::*;
use upcast::Upcast;

use crate::{record::*, unique_allocate::AllocateUniqueFeature, *};

#[salsa::query_group(FeatureQueryGroupStorage)]
pub trait FeatureQueryGroup:
    AllocateUniqueFeature + PackageQueryGroup + Upcast<dyn EntityQueryGroup>
{
    fn main_feature_block(&self, main_file: file::FilePtr) -> SemanticResultArc<FeatureBlock>;
    fn scoped_feature_block(&self, scope: ScopePtr) -> SemanticResultArc<FeatureBlock>;
    fn record_memb_repr(&self, this: FeatureRepr, memb_ident: CustomIdentifier) -> FeatureRepr;
}

fn main_feature_block(
    db: &dyn FeatureQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<FeatureBlock> {
    let package = db.package(main_file)?;
    let main = &*package.main;
    Ok(FeatureBlock::new(db, None, &main.stmts, &[], db.features()))
}

fn scoped_feature_block(
    db: &dyn FeatureQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<FeatureBlock> {
    let entity = db.entity(scope)?;
    match entity.kind() {
        EntityKind::Feature { lazy_stmts, .. } => {
            Ok(FeatureBlock::new(db, None, lazy_stmts, &[], db.features()))
        }
        _ => todo!(),
    }
}
