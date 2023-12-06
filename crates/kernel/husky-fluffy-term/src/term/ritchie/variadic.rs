use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyRitchieVariadicParameter {
    contract: Contract,
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

impl FluffyRitchieVariadicParameter {
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
