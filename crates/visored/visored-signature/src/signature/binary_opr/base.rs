use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdBaseBinaryOprSignature {
    pub instantiation: VdInstantiation,
    pub lopr_ty: VdZfcType,
    pub ropr_ty: VdZfcType,
    pub expr_ty: VdZfcType,
}

impl VdBaseBinaryOprSignature {
    pub fn new(
        instantiation: VdInstantiation,
        lopr_ty: VdZfcType,
        ropr_ty: VdZfcType,
        expr_ty: VdZfcType,
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

    pub fn lopr_ty(self) -> VdZfcType {
        self.lopr_ty
    }

    pub fn ropr_ty(self) -> VdZfcType {
        self.ropr_ty
    }

    pub fn expr_ty(self) -> VdZfcType {
        self.expr_ty
    }
}
