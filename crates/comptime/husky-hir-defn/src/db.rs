use crate::*;

pub trait HirDefnDb: salsa::DbWithJar<HirDefnJar> {}

impl<Db> HirDefnDb for Db where Db: salsa::DbWithJar<HirDefnJar> {}

#[salsa::jar(db = HirDefnDb)]
pub struct HirDefnJar();
