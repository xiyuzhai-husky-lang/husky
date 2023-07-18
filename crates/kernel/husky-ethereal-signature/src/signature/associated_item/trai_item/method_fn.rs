use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitMethodFnEtherealSignatureTemplate {
    pub path: TraitItemPath,
    pub generic_parameters: EtherealGenericParameters,
    /// `Self` as generic parameter
    pub self_ty_generic_parameter: EtherealGenericParameter,
}
