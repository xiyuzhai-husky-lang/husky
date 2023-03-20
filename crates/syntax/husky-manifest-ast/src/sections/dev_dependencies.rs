use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDevDependenciesSectionAst {
    dependencies: Vec<ManifestDependencyAst>,
}

impl<'a> ManifestAstParser<'a, TomlTable> {
    pub(crate) fn parse_dev_dependencies_section(
        &mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<ManifestDevDependenciesSectionAst>> {
        let Some(dev_dependencies_section_ast) = self
            .visit_normal_section_ast(self.menu().dev_dependencies_word(), errors)? else {
            return Ok(None)
        };
        Ok(Some(ManifestDevDependenciesSectionAst {
            dependencies: todo!(), // ad hoc
        }))
    }
}
