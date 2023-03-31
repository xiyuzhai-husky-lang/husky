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
pub enum Signature {
    Type(TypeSignature),
    Form(FormSignature),
    Trait(TraitSignature),
    Impl(ImplSignature),
    AssociatedItem(AssociatedItemSignature),
    Variant(VariantSignature),
    DeriveDecr(DeriveDecrSignature),
}

impl Signature {}

impl From<TypeSignature> for Signature {
    fn from(v: TypeSignature) -> Self {
        Self::Type(v)
    }
}

impl From<FormSignature> for Signature {
    fn from(v: FormSignature) -> Self {
        Self::Form(v)
    }
}

impl From<TraitSignature> for Signature {
    fn from(v: TraitSignature) -> Self {
        Self::Trait(v)
    }
}

impl From<ImplSignature> for Signature {
    fn from(v: ImplSignature) -> Self {
        Self::Impl(v)
    }
}

impl From<AssociatedItemSignature> for Signature {
    fn from(v: AssociatedItemSignature) -> Self {
        Self::AssociatedItem(v)
    }
}

impl From<VariantSignature> for Signature {
    fn from(v: VariantSignature) -> Self {
        Self::Variant(v)
    }
}
