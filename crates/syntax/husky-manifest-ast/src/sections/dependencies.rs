use husky_toml_ast::TomlSection;
use vec_like::VecMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    idx: TomlSectionIdx,
    dependencies: VecMap<ManifestDependencyAst>,
}

impl ManifestDependenciesSectionAst {
    pub fn dependencies(&self) -> &[ManifestDependencyAst] {
        self.dependencies.as_ref()
    }
}

impl<'a> ManifestAstParser<'a, TomlTable> {
    pub(crate) fn parse_dependencies_section(
        &mut self,
        db: &dyn ManifestAstDb,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<ManifestDependenciesSectionAst>> {
        let Some(normal_section_parser)= self.normal_section_parser(
            self.menu().dependencies_word(),
            errors,
        )? else {
            return Ok(None)
        };
        Ok(Some(
            normal_section_parser.parse_into_dependencies_section(errors),
        ))
    }
}

impl<'a> ManifestAstParser<'a, TomlSection> {
    fn parse_into_dependencies_section(
        mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestDependenciesSectionAst {
        //      Ok(Some(ManifestDependenciesSectionAst {
        //     idx,
        //     dependencies: dependencies_section_ast
        //         .entries()
        //         .iter()
        //         .filter_map(|entry| {
        //             self.entry_parser()
        //             ManifestDependencyAst::from_toml_section_entry(db, entry, self.expr_, errors)
        //         })
        //         .collect(), // ad hoc
        // }))
        todo!()
    }
}
