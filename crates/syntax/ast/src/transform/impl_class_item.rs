use super::utils::*;
use crate::*;
use token::{Special, Token, TokenKind};
use vm::MembAccessContract;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_class_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Config(_) => todo!(),
                Keyword::Routine(_) => todo!(),
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Def => self.parse_class_memb_morphism(token_group, enter_block),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_class_memb_var(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::I32Literal(_) => todo!(),
            TokenKind::F32Literal(_) => todo!(),
        }
    }
    fn parse_class_memb_var(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify!(Some(self.file), &token_group[0]);
            let ty = atom::parse_ty(self.symbol_proxy(), &token_group[2..], Some(self.file))?;
            Ok(AstKind::MembVarDefn {
                ident,
                signature: MembAccessDecl {
                    contract: MembAccessContract::Own,
                    ty,
                },
            })
        } else {
            p!(token_group);
            todo!()
        }
    }

    fn parse_class_memb_morphism(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        self.env.set_value(Env::Morphism);
        let ident = identify!(Some(self.file), &token_group[1]);
        let ty = atom::parse_ty(self.symbol_proxy(), &token_group[3..], Some(self.file))?;
        Ok(AstKind::MembFeatureDefnHead { ident, ty })
    }
}
