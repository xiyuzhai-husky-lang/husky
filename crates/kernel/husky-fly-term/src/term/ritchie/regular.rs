use super::*;
use husky_eth_term::term::ritchie::EthRitchieSimpleParameter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyRitchieSimpleParameter {
    pub contract: Contract,
    pub ty: FlyTerm,
}

impl FlyRitchieSimpleParameter {
    pub(super) fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> Option<EthRitchieSimpleParameter> {
        Some(EthRitchieSimpleParameter::new(
            self.contract,
            self.ty.resolve_as_ethereal(terms)?,
        ))
    }
}

impl From<EthRitchieSimpleParameter> for FlyRitchieSimpleParameter {
    fn from(param: EthRitchieSimpleParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FlyInstantiate for EthRitchieSimpleParameter {
    type Target = FlyRitchieSimpleParameter;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        FlyRitchieSimpleParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FlyRitchieSimpleParameter {
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
