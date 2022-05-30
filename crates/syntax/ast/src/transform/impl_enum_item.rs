use crate::*;
use token::*;

use token::identify_token;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_enum_variant(&mut self, tokens: &[Token]) -> AstResult<AstVariant> {
        if tokens.len() == 1 {
            Ok(AstVariant::EnumVariantDefnHead {
                ident: identify_token!(self, tokens[0], SemanticTokenKind::EnumVariant),
                variant_class: EnumVariantKind::Constant,
            })
        } else {
            match tokens[0].kind {
                TokenKind::Keyword(_) => todo!(),
                _ => err!(
                    format!(
                        "expect keyword to lead multiple tokens in definition head of enum variant"
                    ),
                    tokens[0].range
                ),
            }
        }
    }
}
