pub trait HirPreludeDb: salsa::DbWithJar<HirPreludeJar> {}

impl<Db> HirPreludeDb for Db where Db: salsa::DbWithJar<HirPreludeJar> {}

#[salsa::jar(db = HirPreludeDb)]
pub struct HirPreludeJar();
