use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDevDependenciesSectionAst {
    dependencies: Vec<ManifestDependencyAst>,
}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_dev_dependencies_section(
        &mut self,
    ) -> ManifestAstResult<Option<ManifestDevDependenciesSectionAst>> {
        let Some(dev_dependencies_section_ast) = self
            .visit_new_normal_section_ast(self.menu().dev_dependencies_section_tile())? else {
            return Ok(None)
        };
        Ok(Some(ManifestDevDependenciesSectionAst {
            dependencies: vec![], // ad hoc
        }))
    }
}
