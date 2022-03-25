use super::*;
use crate::{transform::utils::*, *};
use word::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_ty_defn(
        &mut self,
        ty_kw: TypeKeyword,
        tokens: &[Token],
    ) -> AstResult<AstKind> {
        match ty_kw {
            TypeKeyword::Struct => self.parse_struct(tokens),
            TypeKeyword::Props => todo!(),
            TypeKeyword::Class => self.parse_class(tokens),
            TypeKeyword::Enum => self.parse_enum(tokens),
            TypeKeyword::Rename => todo!(),
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
        self.env.set_value(Env::Struct);
        expect_len!(Some(self.file), tokens, 3);
        expect_head!(Some(self.file), tokens);
        Ok(AstKind::TypeDecl {
            ident: identify!(Some(self.file), tokens[1]),
            kind: RawTyKind::Struct,
            generics: Vec::new(),
        })
    }

    fn parse_class(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
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
        self.env.set_value(Env::Class);
        expect_len!(Some(self.file), tokens, 3);
        expect_head!(Some(self.file), tokens);
        Ok(AstKind::TypeDecl {
            ident: identify!(Some(self.file), tokens[1]),
            kind: RawTyKind::Class,
            generics: Vec::new(),
        })
    }

    fn parse_enum(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        self.env.set_value(Env::Enum);
        expect_len!(Some(self.file), tokens, 3);
        expect_head!(Some(self.file), tokens);
        Ok(AstKind::TypeDecl {
            ident: identify!(Some(self.file), tokens[1]),
            kind: RawTyKind::Enum,
            generics: Vec::new(),
        })
    }
}
