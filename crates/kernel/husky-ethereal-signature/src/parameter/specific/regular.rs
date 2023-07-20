use super::*;

// todo: redundant
// replace this with ethereal term ritchie parameter
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RegularSpecificParameter {
    contract: Contract,
    ty: EtherealTerm,
}

impl Into<EtherealTermRitchieRegularParameter> for RegularSpecificParameter {
    fn into(self) -> EtherealTermRitchieRegularParameter {
        EtherealTermRitchieRegularParameter::new(self.contract, self.ty)
    }
}

impl Into<EtherealTermRitchieParameter> for RegularSpecificParameter {
    fn into(self) -> EtherealTermRitchieParameter {
        EtherealTermRitchieParameter::Regular(self.into())
    }
}

impl RegularSpecificParameter {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature: SpecificRegularDeclarativeParameterTemplate,
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

impl EtherealTermInstantiate for RegularSpecificParameter {
    type Target = Self;

    fn instantiate(
        self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target {
        RegularSpecificParameter {
            contract: self.contract,
            ty: self.ty.instantiate(db, instantiation),
        }
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
