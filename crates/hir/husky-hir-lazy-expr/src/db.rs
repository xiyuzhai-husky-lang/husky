use crate::{source_map::HirLazyExprSourceMap, *};
use husky_hir_ty::db::HirTypeDb;
use husky_sema_expr::SemaExprDb;

pub trait HirLazyExprDb: salsa::DbWithJar<HirLazyExprJar> + SemaExprDb + HirTypeDb {}

impl HirLazyExprDb for Db where Db: salsa::DbWithJar<HirLazyExprJar> + SemaExprDb + HirTypeDb {}

#[salsa::jar(db = HirLazyExprDb)]
pub struct HirLazyExprJar(
    HirLazyExprRegion,
    HirLazyExprSourceMap,
    hir_lazy_expr_region_with_source_map,
    crate::helpers::control_flow::hir_lazy_expr_region_control_flow,
);
