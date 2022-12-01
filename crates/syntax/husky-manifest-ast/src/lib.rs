use husky_text::TextRange;
use salsa::DbWithJar;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar();

pub trait ManifestAstDb: DbWithJar<ManifestAstJar> {}

pub enum ManifestAst {}
