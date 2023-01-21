use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TupleVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureTermRegion,
}
