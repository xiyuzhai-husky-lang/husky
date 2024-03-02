use super::*;
use crate::signature::binary_opr::SemaBinaryOprFlySignature;

pub type SemaBinaryOprDynamicDispatch = FlyDynamicDispatch<SemaBinaryOprFlySignature>;

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
