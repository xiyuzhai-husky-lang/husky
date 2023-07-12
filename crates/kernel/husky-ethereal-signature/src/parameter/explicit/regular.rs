use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitRegularParameterEtherealSignatureTemplate {
    contract: Contract,
    ty: EtherealTerm,
}

impl ExplicitRegularParameterEtherealSignatureTemplate {
    pub(crate) fn from_declarative_signature_template(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &ExplicitRegularParameterDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            contract: declarative_signature.contract(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature.ty())?,
        })
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

impl EtherealTermInstantiateRef for ExplicitRegularParameterEtherealSignatureTemplate {
    type Target = Self;

    fn instantiate(
        &self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitRegularParameterEtherealSignature {
    contract: Contract,
    ty: EtherealTerm,
}

impl ExplicitRegularParameterEtherealSignature {
    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
