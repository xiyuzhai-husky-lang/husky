use crate::*;
use token::*;
use vm::FieldContract;

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
                Keyword::Def => self.parse_record_derived_field(token_group, enter_block),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_record_original_field(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
        }
    }
    fn parse_record_original_field(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify!(self, &token_group[0], SemanticTokenKind::Field);
            let symbol_context = self.symbol_context();
            let ty = atom::parse_ty(&symbol_context, &token_group[2..])?;
            msg_once!("field contract");
            Ok(AstKind::FieldDefnHead(FieldDefnHead {
                ident,
                contract: FieldContract::Own,
                ty,
                kind: FieldKind::RecordOriginal,
            }))
        } else {
            p!(token_group);
            todo!()
        }
    }

    fn parse_record_derived_field(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        self.env.set_value(AstContext::Morphism);
        let ident = identify!(self, &token_group[1], SemanticTokenKind::Field);
        msg_once!("field contract");
        let ty = atom::parse_ty(&self.symbol_context(), &token_group[3..])?;
        Ok(AstKind::FieldDefnHead(FieldDefnHead {
            ident,
            ty,
            kind: FieldKind::RecordDerived,
            contract: FieldContract::Own,
        }))
    }
}
