use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDevDependenciesSectionAst {
    dependencies: Vec<ManifestDependenyAst>,
}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_dev_dependencies_section(
        &mut self,
    ) -> Option<ManifestDevDependenciesSectionAst> {
        todo!()
    }
}
