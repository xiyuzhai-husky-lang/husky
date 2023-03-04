use crate::*;

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct PropsVariantRawSignature {
    #[return_ref]
    pub term_sheet: RawSignatureTermRegion,
}
