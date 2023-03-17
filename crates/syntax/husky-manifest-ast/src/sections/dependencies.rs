use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    dependencies: Vec<ManifestDependencyAst>,
}

impl ManifestDependenciesSectionAst {
    pub fn dependencies(&self) -> &[ManifestDependencyAst] {
        self.dependencies.as_ref()
    }
}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_dependencies_section(
        &mut self,
    ) -> ManifestAstResult<Option<ManifestDependenciesSectionAst>> {
        let Some(dependencies_section_ast) = self
            .visit_new_normal_section_ast(self.menu().dependencies_section_tile())? else {
            return Ok(None)
        };
        Ok(Some(ManifestDependenciesSectionAst {
            dependencies: vec![], // ad hoc
        }))
    }
}
