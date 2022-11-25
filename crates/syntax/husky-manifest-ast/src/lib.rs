use salsa::DbWithJar;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar();

pub trait ManifestAstDb: DbWithJar<ManifestAstJar> {}
