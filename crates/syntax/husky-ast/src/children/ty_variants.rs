use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeVariants {
    ast_idx_range: AstIdxRange,
}

impl TypeVariants {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl<'a> TryParseOptionFromStreamWithContext<AstParser<'a>> for TypeVariants {
    type Error = std::convert::Infallible;

    type Context = TypePath;

    #[inline(always)]
    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut AstParser<'a>,
        ctx: Self::Context,
    ) -> Result<Option<Self>, std::convert::Infallible> {
        let ast_idx_range = sp.parse_ty_variants(ctx);
        Ok((ast_idx_range.len() > 0).then_some(TypeVariants { ast_idx_range }))
    }
}
