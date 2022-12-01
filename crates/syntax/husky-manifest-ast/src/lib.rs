use husky_text::TextRange;
use salsa::DbWithJar;

#[salsa::jar(db = ManifestAstDb)]
pub struct ManifestAstJar();

pub trait ManifestAstDb: DbWithJar<ManifestAstJar> {}

pub struct ManifestAst {
    variant: ManifestAstVariant,
    range: TextRange,
}

pub enum ManifestAstVariant {}
