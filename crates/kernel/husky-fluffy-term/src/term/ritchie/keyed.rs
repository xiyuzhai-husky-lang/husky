use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieKeyedParameter {
    key: Ident,
    contract: Contract,
    ty: FluffyTerm,
    default: Option<FluffyTerm>,
}

impl From<EtherealTermRitchieKeyedParameter> for FluffyTermRitchieKeyedParameter {
    #[inline(always)]
    fn from(param: EtherealTermRitchieKeyedParameter) -> Self {
        Self {
            key: param.key(),
            contract: param.contract(),
            ty: param.ty().into(),
            default: param.default().map(Into::into),
        }
    }
}

impl FluffyTermRitchieKeyedParameter {
    #[inline(always)]
    pub fn contract(&self) -> Contract {
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
}
