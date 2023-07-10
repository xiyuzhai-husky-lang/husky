use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigRegistrySectionAst {
    path: Option<CorgiConfigAstResult<DiffPath>>,
}

impl CorgiConfigRegistrySectionAst {
    pub fn path(&self) -> Option<CorgiConfigAstResultRef<DiffPath>> {
        self.path.as_ref().map(|s| s.as_ref().copied())
    }
}

impl TransformFromTomlAst<CorgiConfigAstTransformContext> for CorgiConfigRegistrySectionAst {
    type Ast = TomlSection;

    fn transform_from<'a, 'b>(
        mut section_transformer: TomlTransformer<'a, 'b, CorgiConfigAstTransformContext, Self::Ast>,
    ) -> CorgiConfigAstResult<Self> {
        let key = section_transformer.menu().path_coword();
        Ok(CorgiConfigRegistrySectionAst {
            path: section_transformer.transform_value(key),
        })
    }
}

impl TransformFromTomlParentKeyed<CorgiConfigAstTransformContext>
    for CorgiConfigRegistrySectionAst
{
    fn key(
        menu: &<CorgiConfigAstTransformContext as TomlDeserializeContext>::Menu,
    ) -> husky_coword::Coword {
        menu.registry_coword()
    }
}
