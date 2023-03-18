use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestPackageSectionAst {}

impl<'a> ManifestAstBuilder<'a, husky_toml_ast::TomlSectionAst> {
    pub(crate) fn build_package_section(
        &mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<ManifestPackageSectionAst> {
        let package_section_ast = self
            .visit_new_normal_section_ast(self.menu().package_section_tile())?
            .ok_or(OriginalManifestAstError::MissingPackageSection)?;
        Ok(ManifestPackageSectionAst {})
    }
}
