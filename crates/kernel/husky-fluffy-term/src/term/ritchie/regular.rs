use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyRitchieRegularParameter {
    pub contract: TermContract,
    pub ty: FlyTerm,
}

impl FlyRitchieRegularParameter {
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

impl From<EtherealRitchieRegularParameter> for FlyRitchieRegularParameter {
    fn from(param: EtherealRitchieRegularParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FlyInstantiate for EtherealRitchieRegularParameter {
    type Target = FlyRitchieRegularParameter;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        FlyRitchieRegularParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
        }
    }
}

impl FlyRitchieRegularParameter {
    pub fn new(contract: TermContract, ty: FlyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> FlyTerm {
        self.ty
    }

    pub fn ty_mut(&mut self) -> &mut FlyTerm {
        &mut self.ty
    }
}
