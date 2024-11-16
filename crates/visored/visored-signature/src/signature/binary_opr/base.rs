use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdBaseBinaryOprSignature {
    pub instantiation: VdInstantiation,
    pub lopr_ty: VdType,
    pub ropr_ty: VdType,
    pub expr_ty: VdType,
}

impl VdBaseBinaryOprSignature {
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

impl VdBaseBinaryOprSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn lopr_ty(self) -> VdType {
        self.lopr_ty
    }

    pub fn ropr_ty(self) -> VdType {
        self.ropr_ty
    }

    pub fn expr_ty(self) -> VdType {
        self.expr_ty
    }
}
