use crate::*;
use husky_word::Word;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependenyAst {
    name: Word,
    errors: Vec<ManifestAstError>,
}
