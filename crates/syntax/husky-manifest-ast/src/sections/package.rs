use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManifestPackageSectionAst {}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_package_section(&mut self) -> ManifestAstResult<ManifestPackageSectionAst> {
        todo!()
    }
}
