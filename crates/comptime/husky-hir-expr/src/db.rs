use crate::*;
use husky_hir_eager_expr::db::HirEagerExprDb;
use husky_hir_lazy_expr::db::HirLazyExprDb;

pub trait HirExprDb: salsa::DbWithJar<HirExprJar> + HirLazyExprDb + HirEagerExprDb {}

impl<Db> HirExprDb for Db where Db: salsa::DbWithJar<HirExprJar> + HirLazyExprDb + HirEagerExprDb {}

#[salsa::jar(db = HirExprDb)]
pub struct HirExprJar();
