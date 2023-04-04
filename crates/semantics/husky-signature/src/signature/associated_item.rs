mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSignature {
    TypeItem(TypeItemSignature),
    TraitItem(TraitItemSignature),
    TraitForTypeItem(TraitForTypeItemSignature),
}

pub(crate) fn associated_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: AssociatedItemDecl,
) -> SignatureResult<AssociatedItemSignature> {
    match decl {
        AssociatedItemDecl::TypeItem(decl) => {
            ty_item_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitItem(decl) => {
            trai_associated_item_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitForTypeItem(decl) => {
            trai_for_ty_associated_item_signature_from_decl(db, decl).map(|s| s.into())
        } // TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
    }
}

impl AssociatedItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            AssociatedItemSignature::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemSignature::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemSignature::TraitForTypeItem(_) => todo!(),
        }
    }
}
