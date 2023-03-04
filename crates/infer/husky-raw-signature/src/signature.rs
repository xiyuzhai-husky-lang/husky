mod associated_item;
mod form;
mod im;
mod trai;
mod ty;
mod variant;

pub use associated_item::*;
pub use form::*;
pub use im::*;
pub use trai::*;
pub use ty::*;
pub use variant::*;

use crate::*;

pub(crate) fn raw_signature(
    db: &dyn RawSignatureDb,
    decl: Decl,
) -> RawSignatureResultRef<RawSignature> {
    match decl {
        Decl::Type(decl) => ty_raw_signature(db, decl).map(|s| s.into()),
        Decl::Form(decl) => form_raw_signature(db, decl).map(|s| s.into()),
        Decl::Trait(decl) => trai_raw_signature(db, decl).as_ref().map(|s| (*s).into()),
        Decl::Impl(decl) => im_raw_signature(db, decl).map(|s| s.into()),
        Decl::AssociatedItem(decl) => associated_item_raw_signature(db, decl).map(|s| s.into()),
        Decl::Variant(decl) => variant_raw_signature(db, decl).map(|s| s.into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb, jar = RawSignatureJar)]
pub enum RawSignature {
    Type(TypeRawSignature),
    Form(FormRawSignature),
    Trait(TraitRawSignature),
    Impl(ImplRawSignature),
    AssociatedItem(AssociatedItemRawSignature),
    Variant(VariantRawSignature),
}

impl RawSignature {}

impl From<TypeRawSignature> for RawSignature {
    fn from(v: TypeRawSignature) -> Self {
        Self::Type(v)
    }
}

impl From<FormRawSignature> for RawSignature {
    fn from(v: FormRawSignature) -> Self {
        Self::Form(v)
    }
}

impl From<TraitRawSignature> for RawSignature {
    fn from(v: TraitRawSignature) -> Self {
        Self::Trait(v)
    }
}

impl From<ImplRawSignature> for RawSignature {
    fn from(v: ImplRawSignature) -> Self {
        Self::Impl(v)
    }
}

impl From<AssociatedItemRawSignature> for RawSignature {
    fn from(v: AssociatedItemRawSignature) -> Self {
        Self::AssociatedItem(v)
    }
}

impl From<VariantRawSignature> for RawSignature {
    fn from(v: VariantRawSignature) -> Self {
        Self::Variant(v)
    }
}
