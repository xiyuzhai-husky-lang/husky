use token::{Token, TokenKind};

use crate::*;

use super::utils::identify;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_enum_variant(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        Ok(if tokens.len() == 1 {
            AstKind::EnumVariantDefnHead {
                ident: identify!(Some(self.file), tokens[0]),
                variant_class: EnumVariantClass::Constant,
            }
        } else {
            todo!()
        })
    }
}
