use super::*;
use husky_declarative_signature::SpecificVariadicParameterDeclarativeSignatureTemplate;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitVariadicParameterEtherealSignatureTemplate {
    contract: Contract,
    ty: EtherealTerm,
}

impl ExplicitVariadicParameterEtherealSignatureTemplate {
    pub(super) fn from_declarative_signature_template(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &SpecificVariadicParameterDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            contract: declarative_signature.contract(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature.ty())?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitVariadicParameterEtherealSignature {
    contract: Contract,
    ty: EtherealTerm,
}
