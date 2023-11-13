use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ManifestPackageSectionAst {}

impl TransformFromTomlAst<ManifestAstTransformContext> for ManifestPackageSectionAst {
    type Ast = TomlSection;

    fn transform_from<'a, 'b>(
        _parser: TomlTransformer<'a, 'b, ManifestAstTransformContext, Self::Ast>,
    ) -> ManifestAstResult<Self> {
        Ok(ManifestPackageSectionAst {})
    }
}

impl TransformFromTomlParentKeyed<ManifestAstTransformContext> for ManifestPackageSectionAst {
    fn key(menu: &<ManifestAstTransformContext as TomlDeserializeContext>::Menu) -> Coword {
        menu.package_coword()
    }
}
