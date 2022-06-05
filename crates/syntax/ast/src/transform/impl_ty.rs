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
        let opt_base_ty = if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        self.context().opt_subroute(self.db.upcast(), custom_ident)
                    }
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        };
        self.opt_base_ty.set(opt_base_ty);
        self.opt_this_liason.set(None);
        self.context.set(AstContext::Struct {
            opt_base_ty,
            item_context: StructItemContext::OriginalField,
        });
        expect_head!(tokens);
        emsg_once!("struct generic placeholders");
        Ok(AstVariant::TypeDefnHead {
            ident: identify_token!(
                self,
                tokens[1],
                SemanticTokenKind::Entity(EntityKind::Type(TyKind::Struct))
            ),
            kind: TyKind::Struct,
            spatial_parameters: Default::default(),
        })
    }

    fn parse_record(&mut self, tokens: &[Token]) -> AstResult<AstVariant> {
        if tokens.len() >= 2 {
            match tokens[1].kind {
                TokenKind::Identifier(ident) => match ident {
                    Identifier::Custom(custom_ident) => {
                        self.opt_base_ty
                            .set(self.context().opt_subroute(self.db.upcast(), custom_ident));
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
            spatial_parameters: Default::default(),
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
        let base_ty = self
            .context()
            .opt_subroute(self.db.upcast(), ident.ident)
            .unwrap();
        self.context.set(AstContext::Enum(base_ty));
        self.opt_base_ty.set(Some(base_ty));
        self.opt_this_liason.set(None);
        Ok(AstVariant::TypeDefnHead {
            ident,
            kind: TyKind::Enum,
            spatial_parameters: Default::default(),
        })
    }
}
