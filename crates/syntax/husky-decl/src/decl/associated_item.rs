mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum AssociatedItemDecl {
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    TypeAsTraitItem(TypeAsTraitItemDecl),
}

impl From<TypeAsTraitItemDecl> for AssociatedItemDecl {
    fn from(v: TypeAsTraitItemDecl) -> Self {
        Self::TypeAsTraitItem(v)
    }
}

impl From<TraitItemDecl> for AssociatedItemDecl {
    fn from(v: TraitItemDecl) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemDecl> for AssociatedItemDecl {
    fn from(v: TypeItemDecl) -> Self {
        Self::TypeItem(v)
    }
}

impl AssociatedItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TypeAsTraitItem(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TypeAsTraitItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TypeAsTraitItem(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.path(db).map(|path| path.into()),
            AssociatedItemDecl::TraitItem(decl) => Some(decl.path(db).into()),
            AssociatedItemDecl::TypeAsTraitItem(decl) => decl.path(db).map(|path| path.into()),
        }
    }
}
