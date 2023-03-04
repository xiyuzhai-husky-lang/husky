use crate::*;

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct PropsVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureTermRegion,
}
