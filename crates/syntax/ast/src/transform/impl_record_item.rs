use crate::*;
use token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_record_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(paradigm) => match paradigm {
                    Paradigm::LazyFunctional => {
                        self.parse_record_derived_field(token_group, enter_block)
                    }
                    Paradigm::EagerFunctional => todo!(),
                    Paradigm::EagerProcedural => todo!(),
                },
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
                Keyword::Visual => todo!(),
                Keyword::Liason(_) => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_record_original_field(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Decorator(_) => todo!(),
        }
    }

    fn parse_record_original_field(&mut self, token_group: &[Token]) -> AstResult<AstVariant> {
        if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(SpecialToken::Colon)
        {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify_token!(self, &token_group[0], SemanticTokenKind::Field);
            let ty = atom::parse_route(self, &token_group[2..])?;
            emsg_once!("field contract");
            Ok(AstVariant::FieldDefnHead {
                ranged_ident: ident,
                liason: MemberLiason::Immutable,
                ty,
                field_ast_kind: FieldAstKind::RecordOriginal,
            })
        } else {
            p!(token_group);
            todo!()
        }
    }

    fn parse_record_derived_field(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        enter_block(self);
        self.context.set(AstContext::Stmt(Paradigm::LazyFunctional));
        self.opt_this_liason.set(Some(ParameterLiason::Pure));
        let ident = identify_token!(self, &token_group[1], SemanticTokenKind::Field);
        emsg_once!("field contract");
        let ty = atom::parse_route(self, &token_group[3..])?;
        Ok(AstVariant::FieldDefnHead {
            ranged_ident: ident,
            ty,
            field_ast_kind: FieldAstKind::RecordDerived,
            liason: MemberLiason::Immutable,
        })
    }
}
