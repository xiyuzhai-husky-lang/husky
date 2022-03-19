use text::TextRanged;
use token::{Special, Token, TokenKind};
use vm::MembVarContract;

use crate::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_memb_var(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        Ok(
            if tokens.len() >= 2 && tokens[1].kind == TokenKind::Special(Special::Colon) {
                if tokens.len() == 2 {
                    todo!()
                }
                let ident = match tokens[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => err!(
                            Some(self.file),
                            tokens[0].text_range(),
                            "expect custom identifier but got builtin"
                        )?,
                        Identifier::Implicit(_) => err!(
                            Some(self.file),
                            tokens[0].text_range(),
                            "expect implicit identifier but got builtin"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => err!(
                        Some(self.file),
                        tokens[0].text_range(),
                        "expect custom identifier"
                    )?,
                };
                let ty = atom::parse_ty(self.symbol_proxy(), &tokens[2..], Some(self.file))?;
                AstKind::MembVar {
                    ident,
                    signature: MembVarSignature {
                        contract: MembVarContract::Own,
                        ty,
                    },
                }
            } else {
                todo!()
            },
        )
    }
}
