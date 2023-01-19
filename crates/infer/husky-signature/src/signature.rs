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

pub(crate) fn signature(db: &dyn SignatureDb, decl: Decl) -> Signature {
    match decl {
        Decl::Type(decl) => ty_signature(db, decl).into(),
        Decl::Form(decl) => form_signature(db, decl).into(),
        Decl::Trait(decl) => trai_signature(db, decl).into(),
        Decl::ImplBlock(decl) => impl_block_signature(db, decl).into(),
        Decl::AssociatedItem(decl) => associated_item_signature(db, decl).into(),
        Decl::Variant(decl) => variant_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Signature {
    Type(TypeSignature),
    Form(FormSignature),
    Trait(TraitSignature),
    ImplBlock(ImplBlockSignature),
    AssociatedItem(AssociatedItemSignature),
    Variant(VariantSignature),
}

impl Signature {
    pub fn term_sheet<'a>(self, db: &'a dyn SignatureDb) -> &'a SignatureTermSheet {
        match self {
            Signature::Type(signature) => signature.term_sheet(db),
            Signature::Form(signature) => signature.term_sheet(db),
            Signature::Trait(signature) => signature.term_sheet(db),
            Signature::ImplBlock(signature) => signature.term_sheet(db),
            Signature::AssociatedItem(signature) => signature.term_sheet(db),
            Signature::Variant(signature) => signature.term_sheet(db),
        }
    }
}

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

impl From<ImplBlockSignature> for Signature {
    fn from(v: ImplBlockSignature) -> Self {
        Self::ImplBlock(v)
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

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for Signature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            Signature::Type(decl) => f
                .debug_tuple("Type")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Signature::Trait(decl) => f
                .debug_tuple("Trait")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Signature::Form(decl) => f
                .debug_tuple("Form")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Signature::Variant(decl) => f
                .debug_tuple("Variant")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Signature::ImplBlock(decl) => f
                .debug_tuple("ImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Signature::AssociatedItem(decl) => f
                .debug_tuple("AssociatedItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
