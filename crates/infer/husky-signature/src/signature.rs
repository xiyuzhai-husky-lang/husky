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

pub(crate) fn signature_from_decl(db: &dyn SignatureDb, decl: Decl) -> SignatureResult<Signature> {
    match decl {
        Decl::Type(decl) => ty_signature_from_decl(db, decl).map(Into::into),
        Decl::Form(decl) => decl.signature(db).map(Into::into),
        Decl::Trait(decl) => trai_signature_from_decl(db, decl).map(Into::into),
        Decl::Impl(decl) => impl_block_signature_from_decl(db, decl).map(Into::into),
        Decl::AssociatedItem(decl) => associated_item_signature_from_decl(db, decl).map(Into::into),
        Decl::Variant(decl) => variant_signature_from_decl(db, decl).map(Into::into),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
#[enum_class::from_variants]
pub enum Signature {
    Type(TypeSignature),
    Form(FormSignature),
    Trait(TraitSignature),
    Impl(ImplSignature),
    AssociatedItem(AssociatedItemSignature),
    Variant(VariantSignature),
    DeriveDecr(DeriveDecrSignature),
}

pub trait HasSignature: Copy {
    type Signature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature>;
}
