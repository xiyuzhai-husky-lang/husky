use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAliasFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
}
