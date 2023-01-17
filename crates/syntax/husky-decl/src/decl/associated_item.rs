mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for AssociatedItemDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            AssociatedItemDecl::TypeItem(decl) => f
                .debug_tuple("TypeItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemDecl::TraitItem(decl) => f
                .debug_tuple("TraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemDecl::TypeAsTraitItem(decl) => f
                .debug_tuple("TypeAsTraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
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

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TypeAsTraitItem(_) => todo!(),
        }
    }

    pub fn expr_page(self, db: &dyn DeclDb) -> ExprPage {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.expr_page(db),
            AssociatedItemDecl::TraitItem(decl) => decl.expr_page(db),
            AssociatedItemDecl::TypeAsTraitItem(decl) => decl.expr_page(db),
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
