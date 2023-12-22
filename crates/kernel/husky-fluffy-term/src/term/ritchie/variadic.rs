use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyRitchieVariadicParameter {
    contract: TermContract,
    ty: FluffyTerm,
}

impl From<EtherealRitchieVariadicParameter> for FluffyRitchieVariadicParameter {
    fn from(param: EtherealRitchieVariadicParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FluffyInstantiate for EtherealRitchieVariadicParameter {
    type Target = FluffyRitchieVariadicParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &FluffyInstantiation,
    ) -> Self::Target {
        FluffyRitchieVariadicParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FluffyRitchieVariadicParameter {
    pub fn new(contract: TermContract, ty: FluffyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    pub fn ty_mut(&mut self) -> &mut FluffyTerm {
        &mut self.ty
    }
}
