pub trait HirConstantDb: salsa::DbWithJar<HirConstantJar> {}

impl<Db> HirConstantDb for Db where Db: salsa::DbWithJar<HirConstantJar> {}

#[salsa::jar(db = HirConstantDb)]
pub struct HirConstantJar();
