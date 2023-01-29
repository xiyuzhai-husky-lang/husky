use crate::*;

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct UnitVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureTermRegion,
}
