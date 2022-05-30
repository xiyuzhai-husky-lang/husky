use super::*;
use crate::*;
use entity_kind::TyKind;
use token::*;
use word::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_ty_defn(
        &mut self,
        ty_kw: TyKeyword,
        tokens: &[Token],
    ) -> AstResult<AstVariant> {
        match ty_kw {
            TyKeyword::Struct => self.parse_struct(tokens),
            TyKeyword::Props => todo!(),
            TyKeyword::Record => self.parse_record(tokens),
            TyKeyword::Enum => self.parse_enum(tokens),
            TyKeyword::Rename => todo!(),
        }
    }

    fn parse_struct(&mut self, tokens: &[Token]) -> AstResult<AstVariant> {
        if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        let this_ty = self.context().subroute(self.db, custom_ident);
                        self.opt_this_ty.set(Some(this_ty));
                        self.opt_this_liason.set(None);
                    }
                    _ => (),
                },
                _ => (),
            }
        };
        self.context
            .set(AstContext::Struct(StructItemContext::OriginalField));
        expect_head!(tokens);
        emsg_once!("struct generic placeholders");
        Ok(AstVariant::TypeDefnHead {
            ident: identify_token!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Struct))
            ),
            kind: TyKind::Struct,
            generic_parameters: Default::default(),
        })
    }

    fn parse_record(&mut self, tokens: &[Token]) -> AstResult<AstVariant> {
        if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        self.opt_this_ty
                            .set(Some(self.context().subroute(self.db, custom_ident)));
                        self.opt_this_liason.set(None);
                    }
                    _ => (),
                },
                _ => (),
            }
        };
        self.context.set(AstContext::Record);
        expect_len!(tokens, 3);
        expect_head!(tokens);
        emsg_once!("record generic placeholders");
        Ok(AstVariant::TypeDefnHead {
            ident: identify_token!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Record))
            ),
            kind: TyKind::Record,
            generic_parameters: Default::default(),
        })
    }

    fn parse_enum(&mut self, tokens: &[Token]) -> AstResult<AstVariant> {
        expect_len!(tokens, 3);
        expect_head!(tokens);
        emsg_once!("record generic placeholders");
        let ident = identify_token!(
            self,
            tokens[1],
            SemanticTokenKind::Entity(EntityKind::Type(TyKind::Enum))
        );
        let this_ty = self.context().subroute(self.db, ident.ident);
        self.context.set(AstContext::Enum(this_ty));
        self.opt_this_ty.set(Some(this_ty));
        self.opt_this_liason.set(None);
        Ok(AstVariant::TypeDefnHead {
            ident,
            kind: TyKind::Enum,
            generic_parameters: Default::default(),
        })
    }
}
