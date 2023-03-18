use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestDevDependenciesSectionAst {
    dependencies: Vec<ManifestDependencyAst>,
}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSection> {
    pub(crate) fn build_dev_dependencies_section(
        &mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<Option<ManifestDevDependenciesSectionAst>> {
        let Some(dev_dependencies_section_ast) = self
            .visit_new_normal_section_ast(self.menu().dev_dependencies_section_tile())? else {
            return Ok(None)
        };
        Ok(Some(ManifestDevDependenciesSectionAst {
            dependencies: todo!(), // ad hoc
        }))
    }
}
