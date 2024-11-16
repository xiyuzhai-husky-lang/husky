use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdBasePrefixOprSignature {
    pub instantiation: VdInstantiation,
    pub lopr_ty: VdType,
    pub ropr_ty: VdType,
    pub expr_ty: VdType,
}

impl VdBasePrefixOprSignature {
    pub fn new(
        instantiation: VdInstantiation,
        lopr_ty: VdType,
        ropr_ty: VdType,
        expr_ty: VdType,
    ) -> Self {
        Self {
            instantiation,
            lopr_ty,
            ropr_ty,
            expr_ty,
        }
    }
}

impl VdBasePrefixOprSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn opr_ty(self) -> VdType {
        self.ropr_ty
    }

    pub fn expr_ty(self) -> VdType {
        self.expr_ty
    }
}
