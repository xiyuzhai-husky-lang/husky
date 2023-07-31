use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct RecordTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
}
