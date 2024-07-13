#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod ki_control_flow;
pub mod ki_repr;
pub mod linkage_impl;
pub mod pedestal;
pub mod ugly;
pub mod vm_control_flow;

pub use self::ki_control_flow::KiControlFlow;
pub use self::linkage_impl::*;
pub use husky_devsoul_interface_macros::*;

use self::ki_repr::{
    KiArgumentReprInterface, KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface,
};
use husky_value_interface::IsValue;
use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;
use std::convert::Infallible;

#[macro_export]
macro_rules! init_crate {
    () => {
        pub(crate) static __TASK_JAR_INDEX: __HuskyJarIndexOnceCell =
            __HuskyJarIndexOnceCell::new();

        pub fn __set_jar_index(jar_index: __HuskyJarIndex) {
            __TASK_JAR_INDEX.set(jar_index).unwrap();
        }

        pub(crate) fn __jar_index() -> __HuskyJarIndex {
            *__TASK_JAR_INDEX
                .get()
                .expect("`__TASK_JAR_INDEX` is not initialized")
        }

        pub(crate) fn __eval_eager_val_item_with<T>(
            ingredient_index: usize,
            f: fn() -> __KiControlFlow,
        ) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value_static(__dev_eval_context().eval_eager_val_item_with(
                __jar_index(),
                __HuskyIngredientIndex::from_index(ingredient_index),
                f,
            ))
        }

        pub(crate) fn __eval_lazy_val_item<T>(ingredient_index: usize) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value_static(__dev_eval_context().eval_lazy_val_item(
                __jar_index(),
                __HuskyIngredientIndex::from_index(ingredient_index),
            ))
        }

        pub(crate) fn __eval_memo_field_with<Slf, T>(
            slf: &'static Slf,
            ingredient_index: usize,
            f: fn(&'static Slf) -> __KiControlFlow,
        ) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value_static(__dev_eval_context().eval_memo_field_with(
                __jar_index(),
                __HuskyIngredientIndex::from_index(ingredient_index),
                slf,
                f,
            ))
        }

        pub(crate) fn __eval_memo_field_return_ref_with<Slf, T>(
            slf: &'static Slf,
            ingredient_index: usize,
            f: fn(&'static Slf) -> __KiControlFlow,
        ) -> &'static T
        where
            T: 'static,
            &'static T: __FromValue,
        {
            <&'static T as __FromValue>::from_value_static(
                __dev_eval_context().eval_memo_field_with(
                    __jar_index(),
                    __HuskyIngredientIndex::from_index(ingredient_index),
                    slf,
                    f,
                ),
            )
        }
    };
}

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

pub type HuskyJarIndexOnceCell = OnceCell<HuskyJarIndex>;

pub struct DevEvalContext<LinkageImpl: IsLinkageImpl> {
    runtime: &'static dyn IsDevRuntimeDyn<LinkageImpl>,
    pedestal: LinkageImpl::Pedestal,
}

impl<LinkageImpl: IsLinkageImpl> Clone for DevEvalContext<LinkageImpl> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime,
            pedestal: self.pedestal,
        }
    }
}

impl<LinkageImpl: IsLinkageImpl> Copy for DevEvalContext<LinkageImpl> {}

impl<LinkageImpl: IsLinkageImpl> DevEvalContext<LinkageImpl> {
    pub fn new(
        runtime: &'static dyn IsDevRuntimeDyn<LinkageImpl>,
        pedestal: LinkageImpl::Pedestal,
    ) -> Self {
        Self { runtime, pedestal }
    }

    /// builder pattern, returns a new context with the given pedestal
    pub fn with_pedestal(&self, pedestal: LinkageImpl::Pedestal) -> Self {
        Self {
            runtime: self.runtime,
            pedestal,
        }
    }

    pub fn pedestal(&self) -> LinkageImpl::Pedestal {
        self.pedestal
    }

    pub fn eval_eager_val_item_with(
        self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: fn() -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_eager_val_item_dyn(jar_index, ingredient_index, self.pedestal, f)
            .unwrap()
    }

    pub fn eval_lazy_val_item(
        self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_lazy_val_item_dyn(jar_index, ingredient_index, self.pedestal)
            .unwrap()
    }

    pub fn eval_ki_repr_interface_at_generic_pedestal_with(
        &self,
        ki_repr_interface: KiReprInterface,
        generic_pedestal: fn(LinkageImpl::Pedestal) -> LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            KiDomainReprInterface,
            &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        val_argument_reprs: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.runtime.eval_ki_repr_interface_at_generic_pedestal_dyn(
            ki_repr_interface,
            generic_pedestal(self.pedestal),
            gn_generic_wrapper,
            val_argument_reprs,
        )
    }

    pub fn eval_ki_repr_interface(
        self,
        ki_repr_interface: KiReprInterface,
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.runtime
            .eval_ki_repr_interface_dyn(ki_repr_interface, self.pedestal)
    }

    pub fn eval_ki_domain_repr_interface(
        self,
        ki_domain_repr_interface: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, LinkageImpl::Exception> {
        self.runtime
            .eval_ki_domain_repr_interface_dyn(ki_domain_repr_interface, self.pedestal)
    }

    pub fn eval_memo_field_with<Slf>(
        self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static Slf,
        f: fn(&'static Slf) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImpl::Value {
        let slf: &'static std::ffi::c_void = unsafe { std::mem::transmute(slf) };
        let f: fn(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<LinkageImpl> =
            unsafe { std::mem::transmute(f) };
        self.runtime
            .eval_memo_field_with_dyn(jar_index, ingredient_index, self.pedestal, slf, f)
            .unwrap()
    }

    pub fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_val_runtime_constant_dyn(val_runtime_constant)
    }
}

pub trait IsDevRuntime<LinkageImpl: IsLinkageImpl> {
    type StaticSelf: IsDevRuntime<LinkageImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

    /// the computation is done by $f$,
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_at_pedestal_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    /// the computation is done by the runtime
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_at_pedestal(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    /// the computation is done by the runtime
    /// returns `LinkageImplKiControlFlow<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_ki_repr_interface_at_pedestal(
        &self,
        ki_repr: KiReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_ki_domain_repr_interface_at_pedestal(
        &self,
        ki_domain_repr: KiDomainReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> KiControlFlow<(), Infallible, LinkageImpl::Exception>;

    /// the computation is done by `f`
    /// returns `LinkageImplKiControlFlow<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce(KiDomainReprInterface) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_memo_field_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinkageImpl::Value;
}

pub trait IsDevRuntimeDyn<LinkageImpl: IsLinkageImpl> {
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: fn() -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_ki_repr_interface_dyn(
        &self,
        ki_repr: KiReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_ki_domain_repr_interface_dyn(
        &self,
        ki_domain_repr: KiDomainReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> KiControlFlow<(), Infallible, LinkageImpl::Exception>;

    fn eval_ki_repr_interface_at_generic_pedestal_dyn(
        &self,
        ki_repr: KiReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            KiDomainReprInterface,
            &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        val_argument_reprs: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_memo_field_with_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> LinkageImpl::Value;
}

impl<LinkageImpl: IsLinkageImpl, Runtime> IsDevRuntimeDyn<LinkageImpl> for Runtime
where
    Runtime: IsDevRuntime<LinkageImpl>,
{
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: fn() -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.eval_ingredient_at_pedestal_with(jar_index, ingredient_index, pedestal, f)
    }

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: <LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.eval_ingredient_at_pedestal(jar_index, ingredient_index, pedestal)
    }

    fn eval_ki_repr_interface_dyn(
        &self,
        ki_repr_interface: KiReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.eval_ki_repr_interface_at_pedestal(ki_repr_interface, pedestal)
    }

    fn eval_ki_domain_repr_interface_dyn(
        &self,
        ki_domain_repr_interface: KiDomainReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> KiControlFlow<(), Infallible, LinkageImpl::Exception> {
        self.eval_ki_domain_repr_interface_at_pedestal(ki_domain_repr_interface, pedestal)
    }

    fn eval_ki_repr_interface_at_generic_pedestal_dyn(
        &self,
        ki_repr_interface: KiReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            KiDomainReprInterface,
            &[KiArgumentReprInterface],
        ) -> LinkageImplKiControlFlow<LinkageImpl>,
        val_argument_reprs: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.eval_ki_repr_with(
            ki_repr_interface,
            generic_pedestal,
            |ki_domain_repr: KiDomainReprInterface| {
                gn_generic_wrapper(
                    DevEvalContext {
                        runtime: unsafe {
                            std::mem::transmute::<_, &'static dyn IsDevRuntimeDyn<LinkageImpl>>(
                                self as &dyn IsDevRuntimeDyn<LinkageImpl>,
                            )
                        },
                        pedestal: generic_pedestal,
                    },
                    ki_domain_repr,
                    val_argument_reprs,
                )
            },
        )
    }

    fn eval_memo_field_with_dyn(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl> {
        self.eval_memo_field_with(jar_index, ingredient_index, pedestal, slf, f)
    }

    fn eval_val_runtime_constant_dyn(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> <LinkageImpl as IsLinkageImpl>::Value {
        self.eval_val_runtime_constant(val_runtime_constant)
    }
}
