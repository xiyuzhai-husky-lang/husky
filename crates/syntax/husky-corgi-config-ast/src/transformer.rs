use crate::*;

pub(crate) type CorgiConfigAstTransformer<'a, 'b, A: TomlAst> =
    TomlTransformer<'a, 'b, CorgiConfigAstTomlTransformContext, A>;

pub struct CorgiConfigAstTomlTransformContext;

impl TomlDeserializeContext for CorgiConfigAstTomlTransformContext {
    type Db<'a> = dyn CorgiConfigAstDb + 'a;
    type Menu = CorgiConfigAstMenu;
    type Error = CorgiConfigAstError;
}
