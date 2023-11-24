use super::*;
use crate::signature::binary_opr::SemaBinaryOprFluffySignature;

pub type SemaBinaryOprDynamicDispatch = FluffyDynamicDispatch<SemaBinaryOprFluffySignature>;

impl SemaBinaryOprDynamicDispatch {
    // ad hoc
    #[deprecated]
    pub fn builtin() -> Self {
        SemaBinaryOprDynamicDispatch::new(
            FluffyIndirections::new(FluffyPlace::Transient),
            SemaBinaryOprFluffySignature::Builtin,
        )
    }
}
