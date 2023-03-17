use husky_toml_token::TomlTokenIdx;
use husky_word::Word;
use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestPackageSectionAst {}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    dependencies: Vec<ManifestDependenyAst>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDevDependenciesSectionAst {
    dependencies: VecPairMap<Word, (TomlTokenIdx, ManifestDependenyAst)>,
}
