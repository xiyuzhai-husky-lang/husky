use super::*;
use husky_task_interface::IsLinkageImpl;
use husky_value_protocol::presentation::EnumU8ValuePresenter;
use husky_virtual_value::value::Value;

impl IsLinkageImpl for Linkage {
    type Pedestal = ();

    type Value = Value;

    // ad hoc
    type Error = ();

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
