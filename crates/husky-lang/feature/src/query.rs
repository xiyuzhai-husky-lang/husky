use std::sync::Arc;

use scope::ScopePtr;
use semantics::EntityKind;
use semantics::PackageQueryGroup;
use semantics::SemanticResultArc;

use crate::{unique_allocate::AllocateUniqueFeature, *};

#[salsa::query_group(FeatureQueryGroupStorage)]
pub trait FeatureQueryGroup: AllocateUniqueFeature + PackageQueryGroup {
    fn main_feature_block(&self, main_file: file::FilePtr) -> SemanticResultArc<FeatureBlock>;
    fn feature_block(&self, scope: ScopePtr) -> SemanticResultArc<FeatureBlock>;
}

fn main_feature_block(
    this: &dyn FeatureQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<FeatureBlock> {
    let package = this.package(main_file)?;
    let main = &*package.main;
    Ok(Arc::new(FeatureBlock::new(
        &main.stmts,
        &[],
        this.features(),
    )))
}

fn feature_block(this: &dyn FeatureQueryGroup, scope: ScopePtr) -> SemanticResultArc<FeatureBlock> {
    let entity = this.entity(scope)?;
    match entity.kind() {
        EntityKind::Feature(_) => todo!(),
        _ => todo!(),
    }
}
