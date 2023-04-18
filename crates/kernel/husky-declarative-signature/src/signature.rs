mod associated_item;
mod derive_decr;
mod form;
mod impl_block;
mod trai;
mod ty;
mod variant;

pub use self::associated_item::*;
pub use self::derive_decr::*;
pub use self::form::*;
pub use self::impl_block::*;
pub use self::trai::*;
pub use self::ty::*;
pub use self::variant::*;

use crate::*;

pub(crate) fn signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: Decl,
) -> DeclarativeSignatureResult<Signature> {
    match decl {
        Decl::Type(decl) => ty_declarative_signature_from_decl(db, decl).map(Into::into),
        Decl::Form(decl) => decl.declarative_signature(db).map(Into::into),
        Decl::Trait(decl) => trai_declarative_signature_from_decl(db, decl).map(Into::into),
        Decl::Impl(decl) => decl.declarative_signature(db).map(Into::into),
        Decl::AssociatedItem(decl) => {
            associated_item_declarative_signature_from_decl(db, decl).map(Into::into)
        }
        Decl::Variant(decl) => variant_signature_from_decl(db, decl).map(Into::into),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum Signature {
    Type(TypeDeclarativeSignature),
    Form(FormDeclarativeSignature),
    Trait(TraitDeclarativeSignature),
    ImplBlock(ImplBlockDeclarativeSignature),
    AssociatedItem(AssociatedItemDeclarativeSignature),
    Variant(VariantDeclarativeSignature),
    DeriveDecr(DeriveDecrDeclarativeSignature),
}

pub trait HasDeclarativeSignature: Copy {
    type DeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature>;
}
