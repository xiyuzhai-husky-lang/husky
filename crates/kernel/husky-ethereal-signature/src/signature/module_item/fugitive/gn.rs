use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct GnFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
}
