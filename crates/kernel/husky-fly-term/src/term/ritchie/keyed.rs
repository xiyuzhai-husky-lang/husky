use super::*;
use husky_eth_term::term::ritchie::EtherealRitchieKeyedParameter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: FlyTerm,
    has_default: bool,
}

impl From<EtherealRitchieKeyedParameter> for FlyRitchieKeyedParameter {
    #[inline(always)]
    fn from(param: EtherealRitchieKeyedParameter) -> Self {
        Self {
            key: param.key(),
            contract: param.contract(),
            ty: param.ty().into(),
            has_default: param.has_default(),
        }
    }
}

impl FlyInstantiate for EtherealRitchieKeyedParameter {
    type Target = FlyRitchieKeyedParameter;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        FlyRitchieKeyedParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
            key: self.key(),
            has_default: self.has_default(),
        }
    }
}

impl FlyRitchieKeyedParameter {
    #[inline(always)]
    pub fn key(&self) -> Ident {
        self.key
    }

    #[inline(always)]
    pub fn contract(&self) -> Contract {
        self.contract
    }

    #[inline(always)]
    pub fn ty(&self) -> FlyTerm {
        self.ty
    }

    #[inline(always)]
    pub fn ty_mut(&mut self) -> &mut FlyTerm {
        &mut self.ty
    }

    #[inline(always)]
    pub fn has_default(&self) -> bool {
        self.has_default
    }
}
