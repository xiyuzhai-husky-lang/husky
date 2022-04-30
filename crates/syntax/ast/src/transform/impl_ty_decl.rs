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
    ) -> AstResult<AstKind> {
        match ty_kw {
            TyKeyword::Struct => self.parse_struct(tokens),
            TyKeyword::Props => todo!(),
            TyKeyword::Record => self.parse_record(tokens),
            TyKeyword::Enum => self.parse_enum(tokens),
            TyKeyword::Rename => todo!(),
        }
    }

    fn parse_struct(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        self.this
                            .set_value(Some(self.env().subscope(self.db, custom_ident)));
                    }
                    _ => (),
                },
                _ => (),
            }
        };
        self.env.set_value(AstContext::Struct);
        expect_len!(tokens, 3);
        expect_head!(tokens);
        msg_once!("struct generic placeholders");
        Ok(AstKind::TypeDefnHead {
            ident: identify!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Struct))
            ),
            kind: TyKind::Struct,
            generic_placeholders: Default::default(),
        })
    }

    fn parse_record(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        self.this
                            .set_value(Some(self.env().subscope(self.db, custom_ident)));
                    }
                    _ => (),
                },
                _ => (),
            }
        };
        self.env.set_value(AstContext::Record);
        expect_len!(tokens, 3);
        expect_head!(tokens);
        msg_once!("record generic placeholders");
        Ok(AstKind::TypeDefnHead {
            ident: identify!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Record))
            ),
            kind: TyKind::Record,
            generic_placeholders: Default::default(),
        })
    }

    fn parse_enum(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        self.env.set_value(AstContext::Enum);
        expect_len!(tokens, 3);
        expect_head!(tokens);
        msg_once!("record generic placeholders");
        Ok(AstKind::TypeDefnHead {
            ident: identify!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Enum))
            ),
            kind: TyKind::Enum,
            generic_placeholders: Default::default(),
        })
    }
}
