use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigRegistrySectionAst {
    path: Option<CorgiConfigAstResult<String>>,
}

impl CorgiConfigRegistrySectionAst {
    pub fn path(&self) -> Option<CorgiConfigAstResultRef<&String>> {
        self.path.as_ref().map(|s| s.as_ref())
    }
}

impl ParseFromToml for CorgiConfigRegistrySectionAst {
    type Context = CorgiConfigAstTomlParseContext;

    type Ast = TomlSection;

    fn parse_from<'a, 'b>(
        mut parser: TomlParser<'a, 'b, Self::Context, Self::Ast>,
    ) -> Result<Self, <Self::Context as TomlParseContext>::Error> {
        Ok(CorgiConfigRegistrySectionAst { path: todo!() })
    }
}

impl ParseFromTomlParentKeyed for CorgiConfigRegistrySectionAst {
    fn key(menu: &<Self::Context as TomlParseContext>::Menu) -> husky_word::Word {
        menu.registry_word()
    }
}
