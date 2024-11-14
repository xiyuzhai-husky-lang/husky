use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBaseSeparatorSignature {
    item_ty: VdZfcType,
    output_ty: VdZfcType,
}

impl VdBaseSeparatorSignature {
    pub(crate) fn new(item_ty: VdZfcType, output_ty: VdZfcType) -> Self {
        Self { item_ty, output_ty }
    }
}

impl VdBaseSeparatorSignature {
    pub fn item_ty(&self) -> VdZfcType {
        self.item_ty
    }

    pub fn output_ty(&self) -> VdZfcType {
        self.output_ty
    }
}
