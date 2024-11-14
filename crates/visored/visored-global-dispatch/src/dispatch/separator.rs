use super::*;
use visored_opr::separator::VdBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_zfc_ty::ty::VdZfcType;

pub enum VdSeparatorGlobalDispatch {
    Separator {
        base_separator: VdBaseSeparator,
        signature: VdBaseSeparatorSignature,
    },
}
