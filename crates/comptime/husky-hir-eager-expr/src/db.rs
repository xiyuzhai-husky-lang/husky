use crate::*;
use husky_expr_ty::ExprTypeDb;

pub trait HirEagerExprDb: salsa::DbWithJar<HirEagerExprJar> + ExprTypeDb {}

impl<Db> HirEagerExprDb for Db where Db: salsa::DbWithJar<HirEagerExprJar> + ExprTypeDb {}

#[salsa::jar(db = HirEagerExprDb)]
pub struct HirEagerExprJar(HirEagerExprRegion);
