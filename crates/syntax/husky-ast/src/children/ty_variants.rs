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

impl<'a> ParseFromStreamWithContext<AstParser<'a>> for TypeVariants {
    type Error = std::convert::Infallible;

    type Context = TypePath;

    #[inline(always)]
    fn parse_from_without_guaranteed_rollback(
        sp: &mut AstParser<'a>,
        ctx: Self::Context,
    ) -> Result<Option<Self>, std::convert::Infallible> {
        let ast_idx_range = sp.parse_ty_variants_without_rollback(ctx);
        Ok((ast_idx_range.len() > 0).then_some(TypeVariants { ast_idx_range }))
    }
}

// let ast_ctx_kind = AstContextKind::inside_defn(entity_kind, entity_path);
// let (body, body_kind) = {
//     let body = self.parse_asts(ctx.subcontext(ast_ctx_kind));
//     match body.last() {
//         Some(_) => (body, DefnBodyKind::Block),
//         None => match self
//             .token_groups
//             .peek_token_group_of_exact_indent_with_its_first_token(ctx.indent())
//         {
//             Some((_, _, first_noncomment_token)) => match first_noncomment_token {
//                 Token::Punctuation(Punctuation::VERTICAL) => (
//                     self.parse_ty_variants(ctx.subcontext(
//                         AstContextKind::ExpectTypeVariants { ty_path: todo!() },
//                     )),
//                     DefnBodyKind::TypeVariants,
//                 ),
//                 _ => (Default::default(), DefnBodyKind::None),
//             },
//             None => (Default::default(), DefnBodyKind::None),
//         },
//     }
// };
