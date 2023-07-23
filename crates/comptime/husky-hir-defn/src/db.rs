use crate::*;
use husky_hir_decl::db::HirDeclDb;

pub trait HirDefnDb: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

impl<Db> HirDefnDb for Db where Db: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

#[salsa::jar(db = HirDefnDb)]
pub struct HirDefnJar();
