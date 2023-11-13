use crate::*;

pub(crate) type CorgiConfigAstTransformer<'a, 'b, A> =
    TomlTransformer<'a, 'b, CorgiConfigAstTransformContext, A>;

pub struct CorgiConfigAstTransformContext;

impl TomlDeserializeContext for CorgiConfigAstTransformContext {
    type Db<'a> = dyn CorgiConfigAstDb + 'a;
    type Menu = CorgiConfigAstMenu;
    type Error = CorgiConfigAstError;
}
