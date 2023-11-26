use crate::*;

pub(crate) type ManifestAstTransformer<'a, 'b, A> =
    TomlTransformer<'a, 'b, ManifestAstTransformContext, A>;

pub struct ManifestAstTransformContext;

impl TomlDeserializeContext for ManifestAstTransformContext {
    type Menu = ManifestAstMenu;
    type Error = ManifestAstError;
}
