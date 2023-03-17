use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestDependenciesSectionAst {
    dependencies: Vec<ManifestDependenyAst>,
}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_dependencies_section(&mut self) -> Option<ManifestDependenciesSectionAst> {
        todo!()
    }
}
