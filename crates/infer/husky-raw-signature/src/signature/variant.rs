mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

pub(crate) fn variant_raw_signature(
    db: &dyn RawSignatureDb,
    decl: VariantDecl,
) -> RawSignatureResultRef<VariantRawSignature> {
    match decl {
        VariantDecl::Props(_) => todo!(),
        VariantDecl::Unit(_) => todo!(),
        VariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_raw_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb)]
pub enum VariantRawSignature {
    Props(PropsVariantRawSignature),
    Unit(UnitVariantRawSignature),
    Tuple(TupleVariantRawSignature),
}
impl VariantRawSignature {
    pub fn term_sheet<'a>(self, db: &'a dyn RawSignatureDb) -> &'a RawSignatureTermRegion {
        match self {
            VariantRawSignature::Props(raw_signature) => raw_signature.term_sheet(db),
            VariantRawSignature::Unit(raw_signature) => raw_signature.term_sheet(db),
            VariantRawSignature::Tuple(raw_signature) => raw_signature.term_sheet(db),
        }
    }
}
