use crate::*;
use husky_hir_defn::db::HirDefnDb;
use husky_val::ValDb;

pub trait ValReprDb: salsa::DbWithJar<ValReprJar> + ValDb + HirDefnDb {}

impl<Db> ValReprDb for Db where Db: salsa::DbWithJar<ValReprJar> + ValDb + HirDefnDb {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(ValRepr, val_repr_val, ValReprExpansion, val_repr_expansion);
