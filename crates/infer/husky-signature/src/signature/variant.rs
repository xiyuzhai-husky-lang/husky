mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

pub(crate) fn variant_signature(
    db: &dyn SignatureDb,
    decl: VariantDecl,
) -> SignatureResultRef<VariantSignature> {
    match decl {
        VariantDecl::Props(_) => todo!(),
        VariantDecl::Unit(_) => todo!(),
        VariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
pub enum VariantSignature {
    Props(PropsVariantSignature),
    Unit(UnitVariantSignature),
    Tuple(TupleVariantSignature),
}
impl VariantSignature {
    pub fn term_sheet<'a>(self, db: &'a dyn SignatureDb) -> &'a SignatureRawTermRegion {
        match self {
            VariantSignature::Props(signature) => signature.term_sheet(db),
            VariantSignature::Unit(signature) => signature.term_sheet(db),
            VariantSignature::Tuple(signature) => signature.term_sheet(db),
        }
    }
}
