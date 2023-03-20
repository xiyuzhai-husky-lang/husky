use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestPackageSectionAst {}

impl<'a> ManifestAstParser<'a, TomlTable> {
    pub(crate) fn parse_package_section(
        &mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestAstResult<ManifestPackageSectionAst> {
        let normal_section_parser = self
            .normal_section_parser(self.menu().package_word(), errors)?
            .ok_or(OriginalManifestAstError::MissingPackageSection)?;
        Ok(normal_section_parser.parse_into_package_section(errors))
    }
}

impl<'a> ManifestAstParser<'a, TomlSection> {
    fn parse_into_package_section(
        mut self,
        errors: &mut Vec<ManifestAstError>,
    ) -> ManifestPackageSectionAst {
        // todo!()
        ManifestPackageSectionAst {}
    }
}
