use super::*;
use crate::signature::assoc_item::trai_for_ty_item::binary_opr::SemaBinaryOprFlySignature;

pub type SemaBinaryOprDynamicDispatch = FlyInstanceDispatch<SemaBinaryOprFlySignature>;

impl SemaBinaryOprDynamicDispatch {
    // ad hoc
    #[deprecated]
    pub fn builtin() -> Self {
        SemaBinaryOprDynamicDispatch::new(
            FlyIndirections::new(FlyQuary::Transient),
            SemaBinaryOprFlySignature::Builtin,
        )
    }
}
