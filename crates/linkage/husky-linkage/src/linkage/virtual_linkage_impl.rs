use super::*;
use husky_task_interface::IsLinkageImpl;
use husky_value_protocol::presentation::EnumU8ValuePresenter;
use husky_virtual_value::value::Value;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VirtualLinkageImpl(Linkage);

impl std::ops::Deref for VirtualLinkageImpl {
    type Target = Linkage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Linkage> for VirtualLinkageImpl {
    fn from(linkage: Linkage) -> Self {
        Self(linkage)
    }
}

impl IsLinkageImpl for VirtualLinkageImpl {
    type Pedestal = ();

    type Value = Value;

    // ad hoc
    type Exception = ();

    fn eval_ki(
        self,
        ki_repr_interface: husky_task_interface::ugly::__KiReprInterface,
        ctx: husky_task_interface::DevEvalContext<Self>,
        arguments: &[husky_task_interface::ugly::__KiArgumentReprInterface],
    ) -> husky_task_interface::LinkageImplKiControlFlow<Self> {
        todo!()
    }

    fn enum_u8_value_presenter(self) -> EnumU8ValuePresenter {
        todo!()
    }
}
