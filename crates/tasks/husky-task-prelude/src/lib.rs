#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod linkage_impl;
pub mod val_control_flow;
pub mod val_repr;
pub mod value;

pub use self::linkage_impl::*;
pub use husky_task_prelude_macros::*;

use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;
use val_repr::{ValArgumentReprInterface, ValReprInterface};

#[macro_export]
macro_rules! init_crate {
    () => {
        pub(crate) static __TASK_JAR_INDEX: __TaskJarIndexOnceCell = __TaskJarIndexOnceCell::new();

        pub fn __set_jar_index(jar_index: __TaskJarIndex) {
            __TASK_JAR_INDEX.set(jar_index).unwrap();
        }

        pub(crate) fn __jar_index() -> __TaskJarIndex {
            *__TASK_JAR_INDEX
                .get()
                .expect("`__TASK_JAR_INDEX` is not initialized")
        }

        pub(crate) fn __eval_eager_val_item_with<T>(
            ingredient_index: usize,
            f: fn() -> __ValControlFlow,
        ) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value(__dev_eval_context().eval_eager_val_item_with(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
                f,
            ))
        }

        pub(crate) fn __eval_lazy_val_item<T>(ingredient_index: usize) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value(__dev_eval_context().eval_lazy_val_item(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
            ))
        }

        pub(crate) fn __eval_memoized_field_with<Slf, T>(
            slf: &'static Slf,
            ingredient_index: usize,
            f: fn(&'static Slf) -> __ValControlFlow,
        ) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value(__dev_eval_context().eval_memoized_field_with(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
                slf,
                f,
            ))
        }

        pub(crate) fn __eval_memoized_field_return_ref_with<Slf, T>(
            slf: &'static Slf,
            ingredient_index: usize,
            f: fn(&'static Slf) -> __ValControlFlow,
        ) -> &'static T
        where
            T: 'static,
            &'static T: __FromValue,
        {
            <&'static T as __FromValue>::from_value(__dev_eval_context().eval_memoized_field_with(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
                slf,
                f,
            ))
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskJarIndex(ShiftedU32);

impl TaskJarIndex {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TaskIngredientIndex(ShiftedU32);

impl TaskIngredientIndex {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

pub type TaskJarIndexOnceCell = OnceCell<TaskJarIndex>;

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
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        f: fn() -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_eager_val_item_dyn(jar_index, ingredient_index, self.pedestal, f)
            .unwrap()
    }

    pub fn eval_lazy_val_item(
        self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_lazy_val_item_dyn(jar_index, ingredient_index, self.pedestal)
            .unwrap()
    }

    pub fn eval_val_repr_argument(
        self,
        val_repr: &ValArgumentReprInterface,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        match *val_repr {
            ValArgumentReprInterface::Ordinary(val_repr) => self.eval_val_repr(val_repr),
            ValArgumentReprInterface::Keyed(_) => todo!(),
            ValArgumentReprInterface::Variadic(_) => todo!(),
            ValArgumentReprInterface::Branch {
                condition,
                ref stmts,
            } => todo!(),
        }
    }

    pub fn eval_val_repr_at_generic_pedestal_with(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: fn(LinkageImpl::Pedestal) -> LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValControlFlow<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.runtime.eval_value_at_generic_pedestal_dyn(
            val_repr,
            generic_pedestal(self.pedestal),
            gn_generic_wrapper,
            val_argument_reprs,
        )
    }

    pub fn eval_val_repr(
        self,
        val_repr: ValReprInterface,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.runtime.eval_val_repr_dyn(val_repr, self.pedestal)
    }

    pub fn eval_memoized_field_with<Slf>(
        self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        slf: &'static Slf,
        f: fn(&'static Slf) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImpl::Value {
        let slf: &'static std::ffi::c_void = unsafe { std::mem::transmute(slf) };
        let f: fn(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl> =
            unsafe { std::mem::transmute(slf) };
        self.runtime
            .eval_memoized_field_with_dyn(jar_index, ingredient_index, slf, f)
            .unwrap()
    }
}

pub trait IsDevRuntime<LinkageImpl: IsLinkageImpl> {
    type StaticSelf: IsDevRuntime<LinkageImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

    /// the computation is done by $f$,
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_at_pedestal_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    /// the computation is done by the runtime
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_at_pedestal(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    /// the computation is done by the runtime
    /// returns `LinkageImplValControlFlow<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_val_repr_at_pedestal(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    /// the computation is done by `f`
    /// returns `LinkageImplValControlFlow<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_val_repr_with(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn eval_memoized_field_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;
}

pub trait IsDevRuntimeDyn<LinkageImpl: IsLinkageImpl> {
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: fn() -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn eval_val_repr_dyn(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn eval_value_at_generic_pedestal_dyn(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValControlFlow<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn eval_memoized_field_with_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;
}

impl<LinkageImpl: IsLinkageImpl, Runtime> IsDevRuntimeDyn<LinkageImpl> for Runtime
where
    Runtime: IsDevRuntime<LinkageImpl>,
{
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: fn() -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.eval_ingredient_at_pedestal_with(jar_index, ingredient_index, pedestal, f)
    }

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: <LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.eval_ingredient_at_pedestal(jar_index, ingredient_index, pedestal)
    }

    fn eval_val_repr_dyn(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.eval_val_repr_at_pedestal(val_repr, pedestal)
    }

    fn eval_value_at_generic_pedestal_dyn(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValControlFlow<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.eval_val_repr_with(val_repr, generic_pedestal, || {
            gn_generic_wrapper(
                DevEvalContext {
                    runtime: unsafe {
                        std::mem::transmute::<_, &'static dyn IsDevRuntimeDyn<LinkageImpl>>(
                            self as &dyn IsDevRuntimeDyn<LinkageImpl>,
                        )
                    },
                    pedestal: generic_pedestal,
                },
                val_argument_reprs,
            )
        })
    }

    fn eval_memoized_field_with_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        self.eval_memoized_field_with(jar_index, ingredient_index, slf, f)
    }
}
