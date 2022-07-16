use crate::*;
use husky_token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_record_item(
        &mut self,
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        match token_group[0].kind {
            HuskyTokenKind::Keyword(keyword) => match keyword {
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
            HuskyTokenKind::Identifier(_) => self.parse_record_original_field(token_group),
            _ => err!(format!("expect record item"), token_group[0].range),
        }
    }

    fn parse_record_original_field(&mut self, token_group: &[HuskyToken]) -> AstResult<AstVariant> {
        if token_group.len() >= 2
            && token_group[1].kind == HuskyTokenKind::Special(SpecialToken::Colon)
        {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify_token!(self, &token_group[0], SemanticTokenKind::Field);
            let ty = husky_atom::parse_route(self, &token_group[2..])?;
            msg_once!("field contract");
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
        token_group: &[HuskyToken],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        enter_block(self);
        self.context.set(AstContext::Stmt(Paradigm::LazyFunctional));
        self.opt_this_liason.set(Some(ParameterLiason::Pure));
        let ident = identify_token!(self, &token_group[1], SemanticTokenKind::Field);
        msg_once!("field contract");
        let ty = husky_atom::parse_route(self, &token_group[3..])?;
        Ok(AstVariant::FieldDefnHead {
            ranged_ident: ident,
            ty,
            field_ast_kind: FieldAstKind::RecordDerived,
            liason: MemberLiason::Immutable,
        })
    }
}
