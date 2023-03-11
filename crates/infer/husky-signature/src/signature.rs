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

pub(crate) fn signature(db: &dyn SignatureDb, decl: Decl) -> SignatureResult<Signature> {
    match decl {
        Decl::Type(decl) => ty_signature(db, decl).map(Into::into),
        Decl::Form(decl) => form_signature(db, decl).map(Into::into),
        Decl::Trait(decl) => trai_signature(db, decl).map(Into::into),
        Decl::Impl(decl) => im_signature(db, decl).map(Into::into),
        Decl::AssociatedItem(decl) => associated_item_signature(db, decl).map(Into::into),
        Decl::Variant(decl) => variant_signature(db, decl).map(Into::into),
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
