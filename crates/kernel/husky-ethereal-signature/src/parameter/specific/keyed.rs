use super::*;
use husky_declarative_signature::SpecificKeyedParameterDeclarativeSignatureTemplate;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SpecificKeyedParameterEtherealSignatureTemplate {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    default: Option<EtherealTerm>,
}

impl SpecificKeyedParameterEtherealSignatureTemplate {
    pub(super) fn from_declarative_signature_template(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &SpecificKeyedParameterDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            key: declarative_signature.key(),
            contract: declarative_signature.contract(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature.ty())?,
            default: match declarative_signature.default() {
                Some(default) => Some(EtherealTerm::from_declarative(db, default, todo!())?),
                None => None,
            },
        })
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }

    pub fn default(&self) -> Option<EtherealTerm> {
        self.default
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitKeyedParameterEtherealSignature {
    key: Ident,
    contract: Contract,
    ty: EtherealTerm,
    default: Option<EtherealTerm>,
}
