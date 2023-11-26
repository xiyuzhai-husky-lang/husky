use crate::*;

pub(crate) type CorgiConfigAstTransformer<'a, 'b, A> =
    TomlTransformer<'a, 'b, CorgiConfigAstTransformContext, A>;

pub struct CorgiConfigAstTransformContext;

impl TomlDeserializeContext for CorgiConfigAstTransformContext {
    type Menu = CorgiConfigAstMenu;
    type Error = CorgiConfigAstError;
}
