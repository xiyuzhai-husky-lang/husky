use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdAttachSignature {
    Power(VdPowerSignature),
}
impl VdAttachSignature {
    pub fn expr_ty(self) -> VdZfcType {
        match self {
            VdAttachSignature::Power(signature) => signature.expr_ty(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdPowerSignature {
    instantiation: VdInstantiation,
    base_ty: VdZfcType,
    exponent_ty: VdZfcType,
    expr_ty: VdZfcType,
}

impl VdPowerSignature {
    pub fn new(
        instantiation: VdInstantiation,
        base_ty: VdZfcType,
        exponent_ty: VdZfcType,
        expr_ty: VdZfcType,
    ) -> Self {
        Self {
            instantiation,
            base_ty,
            exponent_ty,
            expr_ty,
        }
    }
}

impl VdPowerSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn base_ty(self) -> VdZfcType {
        self.base_ty
    }

    pub fn exponent_ty(self) -> VdZfcType {
        self.exponent_ty
    }

    pub fn expr_ty(self) -> VdZfcType {
        self.expr_ty
    }
}
