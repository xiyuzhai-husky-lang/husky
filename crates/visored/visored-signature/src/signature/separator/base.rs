use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdBaseSeparatorSignature {
    instantiation: VdInstantiation,
    item_ty: VdZfcType,
    expr_ty: VdZfcType,
}

impl VdBaseSeparatorSignature {
    pub(crate) fn new(
        instantiation: VdInstantiation,
        item_ty: VdZfcType,
        expr_ty: VdZfcType,
    ) -> Self {
        Self {
            instantiation,
            item_ty,
            expr_ty,
        }
    }
}

impl VdBaseSeparatorSignature {
    pub fn item_ty(&self) -> VdZfcType {
        self.item_ty
    }

    pub fn expr_ty(&self) -> VdZfcType {
        self.expr_ty
    }
}
