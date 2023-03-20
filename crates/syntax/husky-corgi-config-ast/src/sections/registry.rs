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

impl TransformFromToml<CorgiConfigAstTomlTransformContext> for CorgiConfigRegistrySectionAst {
    type Ast = TomlSection;

    fn transform_from<'a, 'b>(
        mut section_transformer: TomlTransformer<
            'a,
            'b,
            CorgiConfigAstTomlTransformContext,
            Self::Ast,
        >,
    ) -> CorgiConfigAstResult<Self> {
        let key = section_transformer.menu().path_word();
        Ok(CorgiConfigRegistrySectionAst {
            path: section_transformer.transform_value(key),
        })
    }
}

impl TransformFromTomlParentKeyed<CorgiConfigAstTomlTransformContext>
    for CorgiConfigRegistrySectionAst
{
    fn key(
        menu: &<CorgiConfigAstTomlTransformContext as TomlDeserializeContext>::Menu,
    ) -> husky_word::Word {
        menu.registry_word()
    }
}
