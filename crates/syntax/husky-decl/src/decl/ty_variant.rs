mod props;
mod tuple;
mod unit;

pub use self::props::*;
pub use self::tuple::*;
pub use self::unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantNodeDecl {
    Props(PropsVariantNodeDecl),
    Unit(UnitVariantNodeDecl),
    Tuple(TupleVariantNodeDecl),
}

impl TypeVariantNodeDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> TypeVariantNodePath {
        match self {
            TypeVariantNodeDecl::Props(_) => todo!(),
            TypeVariantNodeDecl::Unit(_) => todo!(),
            TypeVariantNodeDecl::Tuple(_) => todo!(),
        }
    }

    pub(crate) fn ast_idx(self, _db: &dyn DeclDb) -> AstIdx {
        todo!()
    }
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
    pub fn node_path(self, db: &dyn DeclDb) -> TypeVariantNodePath {
        match self {
            TypeVariantDecl::Props(_) => todo!(),
            TypeVariantDecl::Unit(_) => todo!(),
            TypeVariantDecl::Tuple(_) => todo!(),
        }
    }

    pub(crate) fn ast_idx(self, _db: &dyn DeclDb) -> AstIdx {
        todo!()
    }
}

impl HasDecl for TypeVariantNodePath {
    type Decl = TypeVariantDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        ty_variant_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_variant_decl(
    db: &dyn DeclDb,
    node_path: TypeVariantNodePath,
) -> DeclResult<TypeVariantDecl> {
    DeclParseContext::new(db, node_path.module_path(db)).parse_ty_variant_decl(node_path)
}

impl HasDecl for TypeVariantPath {
    type Decl = TypeVariantDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        self.node_path(db).decl(db)
    }
}

impl<'a> DeclParseContext<'a> {
    fn parse_ty_variant_decl(&self, id: TypeVariantNodePath) -> DeclResult<TypeVariantDecl> {
        let (
            ast_idx,
            Ast::TypeVariant {
                token_group_idx,
                vertical_token,
                ident_token,
                state_after,
                ..
            }
        ) = self.resolve_ty_variant_indexed_ast(id) else {
            unreachable!()
        };
        let mut parser = self.expr_parser(id, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, *token_group_idx, Some(*state_after));
        Ok(match ctx.next() {
            Some(Token::Punctuation(Punctuation::LPAR)) => todo!(),
            Some(Token::Punctuation(Punctuation::LCURL)) => todo!(),
            None => UnitVariantDecl::new(self.db(), id, parser.finish()).into(),
            _ => todo!(),
        })
    }
}
