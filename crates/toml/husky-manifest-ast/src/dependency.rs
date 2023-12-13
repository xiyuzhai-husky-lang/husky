use crate::*;
use husky_coword::Kebab;
use vec_like::AsVecMapEntry;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependencyAst {
    line_group_idx: TomlLineGroupIdx,
    name: Kebab,
}

impl TransformFromTomlKeyedAst<ManifestAstTransformContext> for ManifestDependencyAst {
    type KeyedAst = TomlSectionEntry;

    fn transform_from<'a, 'b>(
        transformer: TomlTransformer<'a, 'b, ManifestAstTransformContext, Self::KeyedAst>,
    ) -> ManifestAstResult<Self> {
        Ok(ManifestDependencyAst {
            line_group_idx: transformer.line_group_idx(),
            name: Kebab::from_coword(transformer.db(), transformer.key())
                .ok_or(OriginalManifestAstError::InvalidName)?,
        })
    }
}

impl ManifestDependencyAst {
    pub fn line_group_idx(&self) -> TomlLineGroupIdx {
        self.line_group_idx
    }

    pub fn name(&self) -> Kebab {
        self.name
    }
}

impl AsVecMapEntry for ManifestDependencyAst {
    type K = Kebab;

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

// pub(crate) fn from_toml_section_entry<'a>(
//     db: &::salsa::Db,
//     entry: &'a TomlSectionEntry,
//     builder: &mut ManifestAstTransformer<'a, TomlExpr>,
//     errors: &mut Vec<ManifestAstError>,
// ) -> Option<ManifestDependencyAst> {
//     let Some(name) = Name::from_coword(db, entry.key()) else {
//         errors.push(todo!());
//         return None
//     };
//     Some(ManifestDependencyAst {
//         line_group_idx: entry.line_group_idx(),
//         name,
//     })
// }
