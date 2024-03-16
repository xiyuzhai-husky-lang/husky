use super::*;
use husky_eth_term::term::ritchie::EtherealRitchieVariadicParameter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyRitchieVariadicParameter {
    contract: Contract,
    ty: FlyTerm,
}

impl From<EtherealRitchieVariadicParameter> for FlyRitchieVariadicParameter {
    fn from(param: EtherealRitchieVariadicParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FlyInstantiate for EtherealRitchieVariadicParameter {
    type Target = FlyRitchieVariadicParameter;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        FlyRitchieVariadicParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FlyRitchieVariadicParameter {
    pub fn new(contract: Contract, ty: FlyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> FlyTerm {
        self.ty
    }

    pub fn ty_mut(&mut self) -> &mut FlyTerm {
        &mut self.ty
    }
}
