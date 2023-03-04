use crate::*;

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct UnitVariantRawSignature {
    #[return_ref]
    pub term_sheet: RawSignatureRawTermRegion,
}
