mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use super::*;

pub(crate) fn associated_item_raw_signature(
    db: &dyn RawSignatureDb,
    decl: AssociatedItemDecl,
) -> RawSignatureResultRef<AssociatedItemRawSignature> {
    match decl {
        AssociatedItemDecl::TypeItem(decl) => {
            ty_associated_item_raw_signature(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitItem(decl) => {
            trai_associated_item_raw_signature(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TypeAsTraitItem(decl) => {
            ty_as_trai_associated_item_raw_signature(db, decl).map(|s| s.into())
        } // TypeDecl::Enum(decl) => enum_ty_raw_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb, jar = RawSignatureJar)]
pub enum AssociatedItemRawSignature {
    TypeItem(TypeItemRawSignature),
    TraitItem(TraitItemRawSignature),
    TypeAsTraitItem(TypeAsTraitItemRawSignature),
}

impl From<TypeAsTraitItemRawSignature> for AssociatedItemRawSignature {
    fn from(v: TypeAsTraitItemRawSignature) -> Self {
        Self::TypeAsTraitItem(v)
    }
}

impl From<TraitItemRawSignature> for AssociatedItemRawSignature {
    fn from(v: TraitItemRawSignature) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemRawSignature> for AssociatedItemRawSignature {
    fn from(v: TypeItemRawSignature) -> Self {
        Self::TypeItem(v)
    }
}

impl AssociatedItemRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            AssociatedItemRawSignature::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemRawSignature::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemRawSignature::TypeAsTraitItem(_) => todo!(),
        }
    }
}
