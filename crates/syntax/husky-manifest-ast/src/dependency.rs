use crate::*;
use husky_word::Word;
use vec_like::AsVecMapEntry;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependencyAst {
    line_group_idx: TomlLineGroupIdx,
    name: Word,
}
impl ManifestDependencyAst {
    pub(crate) fn from_toml_section_entry<'a>(
        entry: &'a TomlSectionEntry,
        builder: &mut ManifestAstBuilder<'a, TomlSection>,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestDependencyAst {
        ManifestDependencyAst {
            line_group_idx: entry.line_group_idx(),
            name: entry.key(),
        }
    }

    pub fn line_group_idx(&self) -> TomlLineGroupIdx {
        self.line_group_idx
    }

    pub fn name(&self) -> Word {
        self.name
    }
}

impl AsVecMapEntry for ManifestDependencyAst {
    type K = Word;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.name
    }

    fn key_ref(&self) -> &Self::K {
        &self.name
    }
}
