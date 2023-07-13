use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SpecificVariadicParameterDeclarativeSignatureTemplate {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl SpecificVariadicParameterDeclarativeSignatureTemplate {
    pub(crate) fn new(contract: Contract, ty: DeclarativeTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }
}
