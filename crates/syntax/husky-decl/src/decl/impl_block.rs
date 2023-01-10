mod ty_as_trai_impl_block;
mod ty_impl_block;

pub use ty_as_trai_impl_block::*;
pub use ty_impl_block::*;

use super::*;
use husky_entity_tree::{ImplBlock, ImplBlockId};
use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockDecl {
    TypeImplBlock(TypeImplBlockDecl),
    TypeAsTraitImplBlock(TypeAsTraitImplBlockDecl),
}

impl From<TypeAsTraitImplBlockDecl> for ImplBlockDecl {
    fn from(v: TypeAsTraitImplBlockDecl) -> Self {
        Self::TypeAsTraitImplBlock(v)
    }
}

impl From<TypeImplBlockDecl> for ImplBlockDecl {
    fn from(v: TypeImplBlockDecl) -> Self {
        Self::TypeImplBlock(v)
    }
}

impl ImplBlockDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplBlockDecl::TypeImplBlock(decl) => decl.ast_idx(db),
            ImplBlockDecl::TypeAsTraitImplBlock(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        todo!()
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            ImplBlockDecl::TypeImplBlock(decl) => decl.expr_sheet(db),
            ImplBlockDecl::TypeAsTraitImplBlock(decl) => decl.expr_sheet(db),
        }
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for ImplBlockDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            ImplBlockDecl::TypeImplBlock(decl) => f
                .debug_tuple("TypeImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            ImplBlockDecl::TypeAsTraitImplBlock(decl) => f
                .debug_tuple("TypeAsTraitImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
