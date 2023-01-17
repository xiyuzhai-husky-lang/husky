mod associated_item;
mod form;
mod impl_block;
mod trai;
mod ty;
mod variant;

pub use associated_item::*;
pub use form::*;
pub use impl_block::*;
pub use trai::*;
pub use ty::*;
pub use variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDecl),
    Variant(VariantDecl),
}

impl From<AssociatedItemDecl> for Decl {
    fn from(v: AssociatedItemDecl) -> Self {
        Self::AssociatedItem(v)
    }
}

impl From<ImplBlockDecl> for Decl {
    fn from(v: ImplBlockDecl) -> Self {
        Self::ImplBlock(v)
    }
}

impl Decl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            Decl::Type(decl) => decl.ast_idx(db),
            Decl::Form(decl) => decl.ast_idx(db),
            Decl::Trait(decl) => decl.ast_idx(db),
            Decl::ImplBlock(decl) => decl.ast_idx(db),
            Decl::AssociatedItem(decl) => decl.ast_idx(db),
            Decl::Variant(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            Decl::Type(decl) => decl.implicit_parameters(db),
            Decl::Form(decl) => decl.implicit_parameters(db),
            Decl::Trait(decl) => decl.implicit_parameters(db),
            Decl::ImplBlock(decl) => decl.implicit_parameters(db),
            Decl::AssociatedItem(decl) => decl.implicit_parameters(db),
            Decl::Variant(decl) => &[],
        }
    }

    pub fn expr_page(self, db: &dyn DeclDb) -> ExprPage {
        match self {
            Decl::Type(decl) => decl.expr_page(db).into(),
            Decl::Form(decl) => decl.expr_page(db).into(),
            Decl::Trait(decl) => decl.expr_page(db).into(),
            Decl::ImplBlock(decl) => decl.expr_page(db).into(),
            Decl::AssociatedItem(decl) => decl.expr_page(db).into(),
            Decl::Variant(decl) => todo!(),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<EntityPath> {
        match self {
            Decl::Type(decl) => Some(decl.path(db).into()),
            Decl::Form(decl) => Some(decl.path(db).into()),
            Decl::Trait(decl) => Some(decl.path(db).into()),
            Decl::ImplBlock(decl) => None,
            Decl::AssociatedItem(decl) => decl.path(db).map(|path| path.into()),
            Decl::Variant(decl) => todo!(),
        }
    }
}

impl From<TraitDecl> for Decl {
    fn from(v: TraitDecl) -> Self {
        Self::Trait(v)
    }
}

impl From<FormDecl> for Decl {
    fn from(v: FormDecl) -> Self {
        Self::Form(v)
    }
}

impl From<TypeDecl> for Decl {
    fn from(v: TypeDecl) -> Self {
        Self::Type(v)
    }
}

impl From<TraitItemDecl> for Decl {
    fn from(v: TraitItemDecl) -> Self {
        Self::AssociatedItem(v.into())
    }
}

impl From<TypeItemDecl> for Decl {
    fn from(v: TypeItemDecl) -> Self {
        Self::AssociatedItem(v.into())
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for Decl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            Decl::Type(decl) => f
                .debug_tuple("Type")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Trait(decl) => f
                .debug_tuple("Trait")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Form(decl) => f
                .debug_tuple("Form")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::Variant(decl) => f
                .debug_tuple("Variant")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::ImplBlock(decl) => f
                .debug_tuple("ImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Decl::AssociatedItem(decl) => f
                .debug_tuple("AssociatedItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
