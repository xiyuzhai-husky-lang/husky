use scope::ScopePtr;
use semantics_entity::{EntityKind, EntityQueryGroup};
use semantics_error::SemanticResultArc;
use semantics_package::*;
use std::sync::Arc;
use upcast::Upcast;

use crate::{unique_allocate::AllocateUniqueFeature, *};

#[salsa::query_group(FeatureQueryGroupStorage)]
pub trait FeatureQueryGroup:
    AllocateUniqueFeature + PackageQueryGroup + Upcast<dyn EntityQueryGroup>
{
    fn main_block(&self, main_file: file::FilePtr) -> SemanticResultArc<FeatureBlock>;
    fn lazy_block(&self, scope: ScopePtr) -> SemanticResultArc<FeatureBlock>;
}

fn main_block(
    this: &dyn FeatureQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<FeatureBlock> {
    let package = this.package(main_file)?;
    let main = &*package.main;
    Ok(Arc::new(FeatureBlock::new(
        this.upcast(),
        &main.stmts,
        &[],
        this.features(),
    )))
}

fn lazy_block(this: &dyn FeatureQueryGroup, scope: ScopePtr) -> SemanticResultArc<FeatureBlock> {
    let entity = this.entity(scope)?;
    match entity.kind() {
        EntityKind::Feature(_) => todo!(),
        _ => todo!(),
    }
}
