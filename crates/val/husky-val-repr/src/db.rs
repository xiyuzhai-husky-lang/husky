use crate::*;
use husky_hir_defn::db::HirDefnDb;
use husky_linkage::db::LinkageDb;
use husky_val::ValDb;

pub trait ValReprDb: salsa::DbWithJar<ValReprJar> + ValDb + HirDefnDb + LinkageDb {}

impl ValReprDb for Db where Db: salsa::DbWithJar<ValReprJar> + ValDb + HirDefnDb + LinkageDb {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(
    ValRepr,
    val_repr_val,
    // val_item_val_repr,
    ValReprExpansion,
    val_repr_expansion,
);
