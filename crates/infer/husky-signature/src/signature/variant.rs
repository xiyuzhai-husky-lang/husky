mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
#[enum_class::from_variants]
pub enum VariantSignature {
    Props(PropsVariantSignature),
    Unit(UnitVariantSignature),
    Tuple(TupleVariantSignature),
}

pub(crate) fn variant_signature_from_decl(
    _db: &dyn SignatureDb,
    decl: TypeVariantDecl,
) -> SignatureResult<VariantSignature> {
    match decl {
        TypeVariantDecl::Props(_) => todo!(),
        TypeVariantDecl::Unit(_) => todo!(),
        TypeVariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
    }
}

impl VariantSignature {}
