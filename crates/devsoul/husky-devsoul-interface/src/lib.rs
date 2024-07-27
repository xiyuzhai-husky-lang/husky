#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod devsoul;
pub mod item_path;
pub mod ki_control_flow;
pub mod ki_repr;
pub mod linket_impl;
pub mod pedestal;
pub mod static_var;
pub mod ugly;
pub mod vm_control_flow;

pub use self::ki_control_flow::KiControlFlow;
pub use self::linket_impl::*;
pub use husky_devsoul_interface_macros::*;
use item_path::ItemPathIdInterface;

use self::ki_repr::{
    KiArgumentReprInterface, KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface,
};
use husky_value_interface::IsValue;
use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;
use std::convert::Infallible;

// #[macro_export]
// macro_rules! init_crate {
//     () => {
//         pub(crate) fn __eval_eager_val_with<T>(
//             item_path_id_interface: usize,
//             f: fn() -> __KiControlFlow,
//         ) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_eager_val_with(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(item_path_id_interface),
//                 f,
//             ))
//         }

//         pub(crate) fn __eval_lazy_val<T>(item_path_id_interface: usize) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_lazy_val(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(item_path_id_interface),
//             ))
//         }

//         pub(crate) fn __eval_memo_field_with<Slf, T>(
//             slf: &'static Slf,
//             item_path_id_interface: usize,
//             f: fn(&'static Slf) -> __KiControlFlow,
//         ) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_memo_field_with(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(item_path_id_interface),
//                 slf,
//                 f,
//             ))
//         }

//         pub(crate) fn __eval_memo_field_return_ref_with<Slf, T>(
//             slf: &'static Slf,
//             item_path_id_interface: usize,
//             f: fn(&'static Slf) -> __KiControlFlow,
//         ) -> &'static T
//         where
//             T: 'static,
//             &'static T: __FromValue,
//         {
//             <&'static T as __FromValue>::from_value_static(
//                 __dev_eval_context().eval_memo_field_with(
//                     __jar_index(),
//                     __HuskyIngredientIndex::from_index(item_path_id_interface),
//                     slf,
//                     f,
//                 ),
//             )
//         }
//     };
// }

pub struct DevEvalContext<LinketImpl: IsLinketImpl> {
    runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>,
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
    ) -> LinketImpl::Value {
        self.runtime
            .eval_generic_gn_with_dyn(ki_repr_interface, pedestal, Box::new(f))
            .unwrap()
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
    ) -> KiControlFlow<(), Infallible, LinketImpl::Exception> {
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
    type StaticSelf: IsDevRuntime<LinketImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

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
    ) -> KiControlFlow<(), Infallible, LinketImpl::Exception>;

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
    ) -> KiControlFlow<(), Infallible, LinketImpl::Exception>;

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
    ) -> KiControlFlow<(), Infallible, LinketImpl::Exception> {
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
