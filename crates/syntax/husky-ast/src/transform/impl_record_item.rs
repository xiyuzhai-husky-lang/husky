use crate::*;
use husky_token::*;
use husky_word::{Keyword, Paradigm};

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
                Keyword::Use => todo!(),
                _ => {
                    err!(
                        format!("bad keyword for record item: `{:?}`", keyword),
                        token_group[0].range
                    )
                }
            },
            TokenKind::Identifier(_) => self.parse_record_original_field(token_group),
            _ => err!(format!("expect record item"), token_group[0].range),
        }
    }

    fn parse_record_original_field(&mut self, token_group: &[Token]) -> AstResult<AstVariant> {
        if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(SpecialToken::Colon)
        {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify_token!(self, &token_group[0], SemanticTokenKind::Field);
            let ty = husky_atom::parse_route(self, &token_group[2..])?;
            msg_once!("field contract");
            Ok(AstVariant::FieldDefnHead {
                ranged_ident: ident,
                liason: MemberModifier::Immutable,
                field_ty: ty,
                ast_field_kind: AstFieldKind::RecordOriginal,
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
        self.opt_this_liason.set(Some(ParameterModifier::None));
        let ident = identify_token!(self, &token_group[1], SemanticTokenKind::Field);
        msg_once!("field contract");
        let field_ty = husky_atom::parse_route(self, &token_group[3..])?;
        self.context.set(AstContext::Stmt {
            paradigm: Paradigm::LazyFunctional,
            return_context: Some(RawReturnContext {
                opt_return_ty: Some(field_ty),
                kind: RawReturnContextKind::MemoField,
            }),
        });
        Ok(AstVariant::FieldDefnHead {
            ranged_ident: ident,
            field_ty,
            ast_field_kind: AstFieldKind::RecordDerived,
            liason: MemberModifier::Immutable,
        })
    }
}
