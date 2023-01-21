use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct UnitVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureTermRegion,
}
