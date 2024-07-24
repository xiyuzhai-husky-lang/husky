#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod devsoul;
pub mod item_path;
pub mod ki_control_flow;
pub mod ki_repr;
pub mod linket_impl;
pub mod pedestal;
pub mod ugly;
pub mod vm_control_flow;

pub use self::ki_control_flow::KiControlFlow;
pub use self::linket_impl::*;
pub use husky_devsoul_interface_macros::*;

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
//             ingredient_index: usize,
//             f: fn() -> __KiControlFlow,
//         ) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_eager_val_with(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(ingredient_index),
//                 f,
//             ))
//         }

//         pub(crate) fn __eval_lazy_val<T>(ingredient_index: usize) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_lazy_val(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(ingredient_index),
//             ))
//         }

//         pub(crate) fn __eval_memo_field_with<Slf, T>(
//             slf: &'static Slf,
//             ingredient_index: usize,
//             f: fn(&'static Slf) -> __KiControlFlow,
//         ) -> T
//         where
//             T: __FromValue + 'static,
//         {
//             <T as __FromValue>::from_value_static(__dev_eval_context().eval_memo_field_with(
//                 __jar_index(),
//                 __HuskyIngredientIndex::from_index(ingredient_index),
//                 slf,
//                 f,
//             ))
//         }

//         pub(crate) fn __eval_memo_field_return_ref_with<Slf, T>(
//             slf: &'static Slf,
//             ingredient_index: usize,
//             f: fn(&'static Slf) -> __KiControlFlow,
//         ) -> &'static T
//         where
//             T: 'static,
//             &'static T: __FromValue,
//         {
//             <&'static T as __FromValue>::from_value_static(
//                 __dev_eval_context().eval_memo_field_with(
//                     __jar_index(),
//                     __HuskyIngredientIndex::from_index(ingredient_index),
//                     slf,
//                     f,
//                 ),
//             )
//         }
//     };
// }

#[deprecated]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HuskyJarIndex(ShiftedU32);

impl HuskyJarIndex {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

#[deprecated]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct HuskyIngredientIndex(ShiftedU32);

impl HuskyIngredientIndex {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

#[deprecated]
pub type HuskyJarIndexOnceCell = OnceCell<HuskyJarIndex>;

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
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_eager_val_with_dyn(jar_index, ingredient_index, f)
            .unwrap()
    }

    pub fn eval_lazy_val(
        self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_lazy_val_dyn(jar_index, ingredient_index)
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

    pub fn eval_memo_field_with<Slf>(
        self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static Slf,
        f: fn(&'static Slf) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImpl::Value {
        let slf: &'static std::ffi::c_void = unsafe { std::mem::transmute(slf) };
        let f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl> =
            unsafe { std::mem::transmute(f) };
        self.runtime
            .eval_memo_field_with_dyn(jar_index, ingredient_index, slf, f)
            .unwrap()
    }

    pub fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value {
        self.runtime
            .eval_val_runtime_constant_dyn(val_runtime_constant)
    }
}

pub trait IsDevRuntime<LinketImpl: IsLinketImpl> {
    type StaticSelf: IsDevRuntime<LinketImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

    /// the computation is done by $f$,
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: impl FnOnce() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    /// the computation is done by the runtime
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
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

    fn eval_memo_field(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value;
}

pub trait IsDevRuntimeDyn<LinketImpl: IsLinketImpl> {
    fn eval_eager_val_with_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_lazy_val_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
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
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinketImpl::Value;
}

impl<LinketImpl: IsLinketImpl, Runtime> IsDevRuntimeDyn<LinketImpl> for Runtime
where
    Runtime: IsDevRuntime<LinketImpl>,
{
    fn eval_eager_val_with_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: fn() -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_ingredient_with(jar_index, ingredient_index, f)
    }

    fn eval_lazy_val_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_ingredient(jar_index, ingredient_index)
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
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl> {
        self.eval_memo_field(jar_index, ingredient_index, slf, f)
    }

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> <LinketImpl as IsLinketImpl>::Value {
        self.eval_val_runtime_constant(val_runtime_constant)
    }
}
