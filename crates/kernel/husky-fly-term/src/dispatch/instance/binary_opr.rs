use super::*;
use crate::signature::assoc_item::trai_for_ty_item::binary_opr::SemaBinaryOprFlySignature;

pub type SemaBinaryOprInstanceDispatch = FlyInstanceDispatch<SemaBinaryOprFlySignature>;

impl SemaBinaryOprInstanceDispatch {
    // ad hoc
    #[deprecated]
    pub fn builtin() -> Self {
        SemaBinaryOprInstanceDispatch::new(
            FlyIndirections::new(FlyQuary::Transient),
            SemaBinaryOprFlySignature::Builtin,
        )
    }
}
