use crate::*;
use husky_expr_ty::ExprTypeDb;

pub trait HirExprDb: salsa::DbWithJar<HirExprJar> + ExprTypeDb {}

impl<Db> HirExprDb for Db where Db: salsa::DbWithJar<HirExprJar> + ExprTypeDb {}

#[salsa::jar(db = HirExprDb)]
pub struct HirExprJar();
