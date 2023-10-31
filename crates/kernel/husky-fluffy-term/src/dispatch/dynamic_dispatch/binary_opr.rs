use super::*;
use crate::signature::binary_opr::BinaryOprFluffySignature;

pub type BinaryOprDynamicDispatch = FluffyDynamicDispatch<BinaryOprFluffySignature>;

impl BinaryOprDynamicDispatch {
    // ad hoc
    #[deprecated]
    pub fn builtin() -> Self {
        BinaryOprDynamicDispatch::new(
            FluffyTermDynamicDispatchIndirections::new(Place::Transient),
            BinaryOprFluffySignature::Builtin,
        )
    }
}
