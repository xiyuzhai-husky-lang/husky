use crate::*;

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct TupleVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureRawTermRegion,
}
