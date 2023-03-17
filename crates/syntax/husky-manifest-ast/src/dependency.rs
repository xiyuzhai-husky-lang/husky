use crate::*;
use husky_word::Word;

#[salsa::tracked(db = ManifestAstDb, jar= ManifestAstJar)]
pub struct ManifestDependencyAst {
    name: Word,
    #[return_ref]
    errors: Vec<ManifestAstError>,
}
