use crate::*;
use husky_word::Word;

#[salsa::tracked(db = ManifestAstDb, jar= ManifestAstJar)]
pub struct ManifestDependenyAst {
    name: Word,
    #[return_ref]
    errors: Vec<ManifestAstError>,
}
