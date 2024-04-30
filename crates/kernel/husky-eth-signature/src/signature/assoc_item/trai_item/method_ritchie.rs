use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EthSignatureJar)]
pub struct TraitMethodFnEthTemplate {
    pub path: TraitItemPath,
    pub template_parameters: EthTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: EthTemplateParameter,
}
