mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantRawDecl {
    Props(PropsVariantRawDecl),
    Unit(UnitVariantRawDecl),
    Tuple(TupleVariantRawDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantDecl {
    Props(PropsVariantDecl),
    Unit(UnitVariantDecl),
    Tuple(TupleVariantDecl),
}

impl TypeVariantDecl {
    pub(crate) fn ast_idx(&self, _db: &dyn DeclDb) -> AstIdx {
        todo!()
    }
}

impl HasDecl for TypeVariantPath {
    type Decl = TypeVariantDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        ty_variant_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn ty_variant_decl(
    db: &dyn DeclDb,
    path: TypeVariantPath,
) -> DeclResult<TypeVariantDecl> {
    DeclParseContext::new(db, path.module_path(db))?.parse_ty_variant_decl(path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_ty_variant_decl(&self, path: TypeVariantPath) -> DeclResult<TypeVariantDecl> {
        let (
            ast_idx,
            Ast::TypeVariant {
                token_group_idx,
                vertical_token,
                ident_token,
                state_after,
                ..
            }
        ) = self.resolve_ty_variant_indexed_ast(path) else {
            unreachable!()
        };
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, *token_group_idx, Some(*state_after));
        Ok(match ctx.next() {
            Some(Token::Punctuation(Punctuation::LPAR)) => todo!(),
            Some(Token::Punctuation(Punctuation::LCURL)) => todo!(),
            None => UnitVariantDecl::new(self.db(), path, parser.finish()).into(),
            _ => todo!(),
        })
    }
}
