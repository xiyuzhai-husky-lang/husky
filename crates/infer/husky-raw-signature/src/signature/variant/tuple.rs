use crate::*;

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TupleVariantRawSignature {
    #[return_ref]
    pub term_sheet: RawSignatureRawTermRegion,
}
