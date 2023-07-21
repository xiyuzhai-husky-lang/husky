use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieRegularParameter {
    contract: Contract,
    ty: FluffyTerm,
}

impl From<EtherealTermRitchieRegularParameter> for FluffyTermRitchieRegularParameter {
    fn from(param: EtherealTermRitchieRegularParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FluffyTermInstantiate for EtherealTermRitchieRegularParameter {
    type Target = FluffyTermRitchieRegularParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
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
