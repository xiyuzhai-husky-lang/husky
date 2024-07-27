use crate::*;
use husky_entity_path::path::ItemPath;
use husky_sem_expr::SemExprRegion;

pub struct HirLazyVarDeps();

pub enum HirLazyStaticVarDep {
    Item(ItemPath),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprVarDepsRegion {}

#[salsa::tracked(return_ref)]
pub fn hir_lazy_expr_var_deps_region(
    db: &::salsa::Db,
    sem_expr_region: SemExprRegion,
) -> HirLazyExprVarDepsRegion {
    todo!()
}
