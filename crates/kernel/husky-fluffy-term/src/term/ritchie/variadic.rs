use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieVariadicParameter {
    contract: TermContract,
    ty: FluffyTerm,
}

impl From<EtherealRitchieVariadicParameter> for FluffyTermRitchieVariadicParameter {
    fn from(param: EtherealRitchieVariadicParameter) -> Self {
        Self {
            contract: param.contract(),
            ty: param.ty().into(),
        }
    }
}

impl FluffyTermRitchieVariadicParameter {
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
