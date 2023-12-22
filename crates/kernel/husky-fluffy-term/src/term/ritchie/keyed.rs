use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub struct FluffyRitchieKeyedParameter {
    key: Ident,
    contract: TermContract,
    ty: FluffyTerm,
    has_default: bool,
}

impl From<EtherealRitchieKeyedParameter> for FluffyRitchieKeyedParameter {
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

impl FluffyInstantiate for EtherealRitchieKeyedParameter {
    type Target = FluffyRitchieKeyedParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &FluffyInstantiation,
    ) -> Self::Target {
        FluffyRitchieKeyedParameter {
            contract: self.contract(),
            ty: self.ty().instantiate(engine, expr_idx, instantiation),
            key: self.key(),
            has_default: self.has_default(),
        }
    }
}

impl FluffyRitchieKeyedParameter {
    #[inline(always)]
    pub fn key(&self) -> Ident {
        self.key
    }

    #[inline(always)]
    pub fn contract(&self) -> TermContract {
        self.contract
    }

    #[inline(always)]
    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    #[inline(always)]
    pub fn ty_mut(&mut self) -> &mut FluffyTerm {
        &mut self.ty
    }

    #[inline(always)]
    pub fn has_default(&self) -> bool {
        self.has_default
    }
}
