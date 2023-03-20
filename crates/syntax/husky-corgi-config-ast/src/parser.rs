use crate::*;

pub(crate) type CorgiConfigAstParser<'a, 'b, A: TomlAst> =
    TomlParser<'a, 'b, CorgiConfigAstTomlParseContext, A>;

pub struct CorgiConfigAstTomlParseContext;

impl TomlParseContext for CorgiConfigAstTomlParseContext {
    type Db<'a> = dyn CorgiConfigAstDb + 'a;
    type Menu = CorgiConfigAstMenu;
    type Error = CorgiConfigAstError;
}
