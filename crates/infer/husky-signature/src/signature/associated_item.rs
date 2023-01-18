mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

pub(crate) fn associated_item_signature(db: &dyn SignatureDb, decl: AssociatedItemDecl) -> AssociatedItemSignature {
    match decl {
        AssociatedItemDecl::TypeItem(_) => todo!(),
        AssociatedItemDecl::TraitItem(_) => todo!(),
        AssociatedItemDecl::TypeAsTraitItem(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AssociatedItemSignature {
    TypeItem(TypeItemSignature),
    TraitItem(TraitItemSignature),
    TypeAsTraitItem(TypeAsTraitItemSignature),
}

impl From<TypeAsTraitItemSignature> for AssociatedItemSignature {
    fn from(v: TypeAsTraitItemSignature) -> Self {
        Self::TypeAsTraitItem(v)
    }
}

impl From<TraitItemSignature> for AssociatedItemSignature {
    fn from(v: TraitItemSignature) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemSignature> for AssociatedItemSignature {
    fn from(v: TypeItemSignature) -> Self {
        Self::TypeItem(v)
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for AssociatedItemSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            AssociatedItemSignature::TypeItem(decl) => f
                .debug_tuple("TypeItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemSignature::TraitItem(decl) => f
                .debug_tuple("TraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            AssociatedItemSignature::TypeAsTraitItem(decl) => f
                .debug_tuple("TypeAsTraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}

impl AssociatedItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            AssociatedItemSignature::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemSignature::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemSignature::TypeAsTraitItem(_) => todo!(),
        }
    }
}
