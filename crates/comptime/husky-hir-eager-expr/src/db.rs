use crate::*;
use husky_hir_ty::db::HirTypeDb;
use husky_sema_expr::SemaExprDb;

pub trait HirEagerExprDb: salsa::DbWithJar<HirEagerExprJar> + SemaExprDb + HirTypeDb {}

impl<Db> HirEagerExprDb for Db where Db: salsa::DbWithJar<HirEagerExprJar> + SemaExprDb + HirTypeDb {}

#[salsa::jar(db = HirEagerExprDb)]
pub struct HirEagerExprJar(HirEagerExprRegion, HirEagerExprSourceMap);
