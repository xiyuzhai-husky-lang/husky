use std::{
    convert::Infallible,
    sync::{Arc, MutexGuard},
};

use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface};
use husky_value::ki_control_flow::KiControlFlow;

use crate::linket_impl::{
    IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedExceptedValue,
    LinketImplTrackedException,
};

pub struct DevEvalContext<LinketImpl: IsLinketImpl> {
    runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>,
}

pub struct DevEvalContextGuard {
    ignore: bool,
    emit: bool,
    unset: unsafe fn(),
}

impl DevEvalContextGuard {
    pub fn new(unset: unsafe fn()) -> Self {
        Self {
            ignore: false,
            emit: false,
            unset,
        }
    }
}

impl DevEvalContextGuard {
    pub fn ignore(&mut self) {
        self.ignore = true
    }

    pub fn emit(&mut self) {
        self.emit = true
    }
}

impl std::ops::Drop for DevEvalContextGuard {
    fn drop(&mut self) {
        if !self.ignore {
            unsafe { (self.unset)() }
        } else {
            if self.emit {
                panic!("drop ignored")
            }
        }
    }
}

impl<LinketImpl: IsLinketImpl> std::fmt::Debug for DevEvalContext<LinketImpl> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevEvalContext")
            .field(
                "runtime",
                &(self.runtime as *const dyn IsDevRuntimeDyn<LinketImpl>),
            )
            .finish()
    }
}

unsafe impl<LinketImpl> Sync for DevEvalContext<LinketImpl> where LinketImpl: IsLinketImpl {}

impl<LinketImpl: IsLinketImpl> Clone for DevEvalContext<LinketImpl> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime,
        }
    }
}

impl<LinketImpl: IsLinketImpl> Copy for DevEvalContext<LinketImpl> {}

impl<LinketImpl: IsLinketImpl> DevEvalContext<LinketImpl> {
    pub fn new(runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>) -> Self {
        Self { runtime }
    }

    pub fn eval_eager_val_with(
        self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_eager_val_with_dyn(item_path_id_interface, pedestal, f)
            .unwrap()
    }

    pub fn eval_lazy_val(
        self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_lazy_val_dyn(item_path_id_interface, pedestal)
            .unwrap()
    }

    pub fn eval_generic_gn_with(
        self,
        ki_repr_interface: KiReprInterface,
        pedestal: LinketImpl::Pedestal,
        f: impl FnOnce() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.runtime
            .eval_generic_gn_with_dyn(ki_repr_interface, pedestal, Box::new(f))
    }

    pub fn eval_ki_repr_interface(
        self,
        ki_repr_interface: KiReprInterface,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.runtime.eval_ki_repr_interface_dyn(ki_repr_interface)
    }

    pub fn eval_ki_domain_repr_interface(
        self,
        ki_domain_repr_interface: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>> {
        self.runtime
            .eval_ki_domain_repr_interface_dyn(ki_domain_repr_interface)
    }

    pub fn eval_memo_field_with<__Self>(
        self,
        item_path_id_interface: ItemPathIdInterface,
        __self: &'static __Self,
        f: fn(&'static __Self) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImpl::Value {
        let slf: &'static std::ffi::c_void = unsafe { std::mem::transmute(__self) };
        let f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl> =
            unsafe { std::mem::transmute(f) };
        self.runtime
            .eval_memo_field_with_dyn(item_path_id_interface, slf, f)
            .unwrap()
    }

    pub fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_val_runtime_constant_dyn(val_runtime_constant)
    }

    pub fn eval_ki_pedestal(self, ki_repr: KiReprInterface) -> LinketImpl::Pedestal {
        self.runtime.eval_ki_pedestal_dyn(ki_repr)
    }
}

pub trait IsDevRuntime<LinketImpl: IsLinketImpl> {
    type ThawedSelf: IsDevRuntime<LinketImpl> + 'static;

    unsafe fn cast_to_thawed_self_static_ref(&self) -> &'static Self::ThawedSelf;

    fn eval_eager_val_with(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_lazy_val(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    /// the computation is done by the runtime
    /// returns `LinketImplKiControlFlow<LinketImpl>` because there is not guaranteed to be no control flow
    fn eval_ki_repr_interface(
        &self,
        ki_repr: KiReprInterface,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_ki_domain_repr_interface(
        &self,
        ki_domain_repr: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>>;

    /// the computation is done by `f`
    /// returns `LinketImplKiControlFlow<LinketImpl>` because there is not guaranteed to be no control flow
    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        f: impl FnOnce(KiDomainReprInterface) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_memo_field_with(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value;

    fn eval_ki_pedestal(&self, ki_repr_interface: KiReprInterface) -> LinketImpl::Pedestal;

    fn eval_generic_gn_with<'a>(
        &'a self,
        ki_repr_interface: KiReprInterface,
        pedestal: <LinketImpl as IsLinketImpl>::Pedestal,
        f: Box<dyn FnOnce() -> LinketImplKiControlFlow<LinketImpl> + 'a>,
    ) -> LinketImplKiControlFlow<LinketImpl>;
}

pub trait IsDevRuntimeDyn<LinketImpl: IsLinketImpl> {
    fn eval_eager_val_with_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_lazy_val_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_generic_gn_with_dyn<'a>(
        &'a self,
        ki_repr_interface: KiReprInterface,
        pedestal: LinketImpl::Pedestal,
        f: Box<dyn FnOnce() -> LinketImplKiControlFlow<LinketImpl> + 'a>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_ki_repr_interface_dyn(
        &self,
        ki_repr: KiReprInterface,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_ki_domain_repr_interface_dyn(
        &self,
        ki_domain_repr: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>>;

    fn eval_memo_field_with_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value;

    fn eval_ki_pedestal_dyn(&self, ki_repr_interface: KiReprInterface) -> LinketImpl::Pedestal;
}

impl<LinketImpl: IsLinketImpl, Runtime> IsDevRuntimeDyn<LinketImpl> for Runtime
where
    Runtime: IsDevRuntime<LinketImpl>,
{
    fn eval_eager_val_with_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_eager_val_with(item_path_id_interface, pedestal, f)
    }

    fn eval_lazy_val_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_lazy_val(item_path_id_interface, pedestal)
    }

    fn eval_ki_repr_interface_dyn(
        &self,
        ki_repr_interface: KiReprInterface,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_ki_repr_interface(ki_repr_interface)
    }

    fn eval_ki_domain_repr_interface_dyn(
        &self,
        ki_domain_repr_interface: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>> {
        self.eval_ki_domain_repr_interface(ki_domain_repr_interface)
    }

    fn eval_memo_field_with_dyn(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_memo_field_with(item_path_id_interface, slf, f)
    }

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> <LinketImpl as IsLinketImpl>::Value {
        self.eval_val_runtime_constant(val_runtime_constant)
    }

    fn eval_ki_pedestal_dyn(&self, ki_repr_interface: KiReprInterface) -> LinketImpl::Pedestal {
        self.eval_ki_pedestal(ki_repr_interface)
    }

    fn eval_generic_gn_with_dyn<'a>(
        &'a self,
        ki_repr_interface: KiReprInterface,
        pedestal: <LinketImpl as IsLinketImpl>::Pedestal,
        f: Box<dyn FnOnce() -> LinketImplKiControlFlow<LinketImpl> + 'a>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_generic_gn_with(ki_repr_interface, pedestal, f)
    }
}
