use super::*;

#[salsa::interned]
pub struct VdBaseSeparatorSignature {
    pub instantiation: VdInstantiation,
    pub item_ty: VdZfcType,
    pub expr_ty: VdZfcType,
}
