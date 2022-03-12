use std::sync::Arc;

use scope::ScopePtr;
use semantics::EntityKind;
use semantics::SemanticQueryGroup;
use semantics::SemanticResultArc;

use crate::{unique_allocate::AllocateUniqueFeature, *};

#[salsa::query_group(FeatureQueryGroupStorage)]
pub trait FeatureQueryGroup:
    AllocateUniqueFeature + SemanticQueryGroup + Upcast<dyn SemanticQueryGroup>
{
    fn main_block(&self, main_file: file::FilePtr) -> SemanticResultArc<LazyBlock>;
    fn lazy_block(&self, scope: ScopePtr) -> SemanticResultArc<LazyBlock>;
}

fn main_block(
    this: &dyn FeatureQueryGroup,
    main_file: file::FilePtr,
) -> SemanticResultArc<LazyBlock> {
    let package = this.package(main_file)?;
    let main = &*package.main;
    Ok(Arc::new(LazyBlock::new(
        this.upcast(),
        &main.stmts,
        &[],
        this.features(),
    )))
}

fn lazy_block(this: &dyn FeatureQueryGroup, scope: ScopePtr) -> SemanticResultArc<LazyBlock> {
    let entity = this.entity(scope)?;
    match entity.kind() {
        EntityKind::Feature(_) => todo!(),
        _ => todo!(),
    }
}
