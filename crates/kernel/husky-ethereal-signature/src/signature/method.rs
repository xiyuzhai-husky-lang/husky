use super::*;

pub trait HasMethodEtherealSignature: Copy {
    type EtherealMethodSignature;

    fn method_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        ty_template_arguments: &[EtherealTerm],
        method_template_arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<Self::EtherealMethodSignature>;
}

impl HasMethodEtherealSignature for TypePath {
    type EtherealMethodSignature = EtherealMethodSignature;

    fn method_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        ty_template_arguments: &[EtherealTerm],
        method_template_arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<Self::EtherealMethodSignature> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EtherealMethodSignature {}
