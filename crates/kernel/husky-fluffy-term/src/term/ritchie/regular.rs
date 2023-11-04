use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieRegularParameter {
    contract: Contract,
    ty: FluffyTerm,
}

impl FluffyTermRitchieRegularParameter {
    pub(super) fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> Option<EtherealRitchieRegularParameter> {
        Some(EtherealRitchieRegularParameter::new(
            self.contract,
            self.ty.resolve_as_ethereal(terms)?,
        ))
    }
}

impl From<EtherealRitchieRegularParameter> for FluffyTermRitchieRegularParameter {
    fn from(param: EtherealRitchieRegularParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FluffyTermInstantiate for EtherealRitchieRegularParameter {
    type Target = FluffyTermRitchieRegularParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        FluffyTermRitchieRegularParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FluffyTermRitchieRegularParameter {
    pub fn new(contract: Contract, ty: FluffyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    pub fn ty_mut(&mut self) -> &mut FluffyTerm {
        &mut self.ty
    }
}
