use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitMethodFnEthTemplate {
    pub path: TraitItemPath,
    pub template_parameters: EtherealTemplateParameters,
    /// `Self` as generic parameter
    pub self_ty_template_parameter: EtherealTemplateParameter,
}
