mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum AssociatedItemDeclarativeSignature {
    TypeItem(TypeItemDeclarativeSignature),
    TraitItem(TraitItemDeclarativeSignature),
    TraitForTypeItem(TraitForTypeItemDeclarativeSignature),
}

pub(crate) fn associated_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: AssociatedItemDecl,
) -> DeclarativeSignatureResult<AssociatedItemDeclarativeSignature> {
    match decl {
        AssociatedItemDecl::TypeItem(decl) => {
            ty_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitItem(decl) => {
            trai_associated_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitForTypeItem(decl) => {
            trai_for_ty_associated_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        } // TypeDecl::Enum(decl) => enum_ty_declarative_signature(db, decl).into(),
    }
}

impl AssociatedItemDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            AssociatedItemDeclarativeSignature::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDeclarativeSignature::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDeclarativeSignature::TraitForTypeItem(_) => todo!(),
        }
    }
}
