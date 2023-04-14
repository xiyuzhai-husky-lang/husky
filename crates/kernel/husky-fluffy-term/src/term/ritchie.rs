use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTermRitchieParameterContractedType {
    contract: Contract,
    ty: FluffyTerm,
}

impl From<TermRitchieParameterContractedType> for FluffyTermRitchieParameterContractedType {
    fn from(contracted_ty: TermRitchieParameterContractedType) -> Self {
        Self {
            contract: contracted_ty.contract(),
            ty: contracted_ty.ty().into(),
        }
    }
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
