use std::num::Saturating;

use self::vm_control_flow::LinketImplVmControlFlow;
use crate::*;
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use pedestal::{IsPedestal, IsPedestalFull};
use serde::Serialize;

pub trait IsLinketImpl: Send + Copy + 'static {
    type Pedestal: IsPedestalFull;
    type Value: IsValue;
    type Exception: std::fmt::Debug + Serialize;

    ///
    fn init_item_path_id_interface(self, item_path_id_interface: ItemPathIdInterface);

    /// assumed that pedestal has already been
    fn eval_ki(
        self,
        ki_repr_interface: KiReprInterface,
        arguments: &[KiArgumentReprInterface],
        ctx: DevEvalContext<Self>,
    ) -> LinketImplKiControlFlow<Self>;

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinketImplVmControlFlow<Self>;

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter;

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId;
}

pub type LinketImplKiControlFlow<LinketImpl, C = <LinketImpl as IsLinketImpl>::Value> =
    KiControlFlow<C, <LinketImpl as IsLinketImpl>::Value, <LinketImpl as IsLinketImpl>::Exception>;

pub enum VmArgumentValue<LinketImpl: IsLinketImpl> {
    Simple(LinketImpl::Value),
    Variadic(Vec<LinketImpl::Value>),
}
