use crate::*;
use token::*;

use super::utils::identify;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_enum_variant(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        Ok(if tokens.len() == 1 {
            AstKind::EnumVariantDefnHead {
                ident: identify!(self, tokens[0], SemanticTokenKind::EnumVariant),
                variant_class: EnumVariantKind::Constant,
            }
        } else {
            todo!()
        })
    }
}
