use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieParameterContractedType {
    contract: Contract,
    ty: FluffyTerm,
}

impl FluffyTermRitchieParameterContractedType {
    #[inline(always)]
    pub fn new(contract: Contract, ty: FluffyTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(self) -> Contract {
        self.contract
    }

    pub fn ty(self) -> FluffyTerm {
        self.ty
    }
}
