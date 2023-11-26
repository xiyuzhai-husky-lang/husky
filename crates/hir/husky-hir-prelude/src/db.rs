pub trait HirPreludeDb: salsa::DbWithJar<HirPreludeJar> {}

impl HirPreludeDb for Db where Db: salsa::DbWithJar<HirPreludeJar> {}

#[salsa::jar(db = HirPreludeDb)]
pub struct HirPreludeJar();
