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

        pub(crate) fn __eager_eval_val_item<T>(ingredient_index: usize, f: fn() -> __Value) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value(__dev_eval_context().eval_eager_val_item(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
                f,
            ))
        }

        pub(crate) fn __lazy_eval_val_item<T>(ingredient_index: usize) -> T
        where
            T: __FromValue + 'static,
        {
            <T as __FromValue>::from_value(__dev_eval_context().eval_lazy_val_item(
                __jar_index(),
                __TaskIngredientIndex::from_index(ingredient_index),
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

    pub fn pedestal(&self) -> LinkageImpl::Pedestal {
        self.pedestal
    }

    pub fn eval_eager_val_item(
        self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        f: fn() -> LinkageImpl::Value,
    ) -> LinkageImpl::Value {
        self.runtime.eval_eager_val_item_dyn(
            jar_index,
            ingredient_index,
            self.pedestal,
            Box::new(move || {
                f();
                todo!()
            }),
        )
    }

    pub fn eval_lazy_val_item(
        self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
    ) -> LinkageImpl::Value {
        self.runtime
            .eval_lazy_val_item_dyn(jar_index, ingredient_index, self.pedestal)
    }

    fn memoized_field(self) -> LinkageImpl::Value {
        todo!()
    }

    pub fn eval_val_repr_argument(
        self,
        val_repr: &ValArgumentReprInterface,
    ) -> LinkageImplValControlFlow<LinkageImpl> {
        todo!()
    }

    pub fn eval_value_at_generic_pedestal(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            LinkageImpl::Pedestal,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult<LinkageImpl> {
        self.runtime.eval_value_at_generic_pedestal_dyn(
            val_repr,
            generic_pedestal,
            gn_generic_wrapper,
            val_argument_reprs,
        )
    }
}

pub trait IsDevRuntime<LinkageImpl: IsLinkageImpl> {
    type StaticSelf: IsDevRuntime<LinkageImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

    /// the computation is done by $f$,
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient_with(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValueResult<LinkageImpl>,
    ) -> LinkageImpl::Value;

    /// the computation is done by the runtime
    /// returns `Value` because there is guaranteed to be no control flow
    fn eval_ingredient(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImpl::Value;

    /// the computation is done by the runtime
    /// returns `LinkageImplValueResult<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_val_repr(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValueResult<LinkageImpl>;

    /// the computation is done by `f`
    /// returns `LinkageImplValueResult<LinkageImpl>` because there is not guaranteed to be no control flow
    fn eval_val_repr_with(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValueResult<LinkageImpl>,
    ) -> LinkageImplValueResult<LinkageImpl>;
}

pub trait IsDevRuntimeDyn<LinkageImpl: IsLinkageImpl> {
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        f: Box<dyn FnOnce() -> LinkageImplValueResult<LinkageImpl>>,
    ) -> LinkageImpl::Value;

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImpl::Value;

    fn eval_val_repr_dyn(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValueResult<LinkageImpl>;

    fn eval_value_at_generic_pedestal_dyn(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            LinkageImpl::Pedestal,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult<LinkageImpl>;
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
        f: Box<dyn FnOnce() -> LinkageImplValueResult<LinkageImpl>>,
    ) -> LinkageImpl::Value {
        self.eval_ingredient_with(jar_index, ingredient_index, pedestal, f)
    }

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: <LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> <LinkageImpl as IsLinkageImpl>::Value {
        self.eval_ingredient(jar_index, ingredient_index, pedestal)
    }

    fn eval_val_repr_dyn(
        &self,
        val_repr: ValReprInterface,
        pedestal: LinkageImpl::Pedestal,
    ) -> LinkageImplValueResult<LinkageImpl> {
        self.eval_val_repr(val_repr, pedestal)
    }

    fn eval_value_at_generic_pedestal_dyn(
        &self,
        val_repr: ValReprInterface,
        generic_pedestal: LinkageImpl::Pedestal,
        gn_generic_wrapper: fn(
            LinkageImpl::Pedestal,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult<LinkageImpl>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult<LinkageImpl> {
        self.eval_val_repr_with(val_repr, generic_pedestal, || {
            gn_generic_wrapper(generic_pedestal, val_argument_reprs)
        })
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! all_ritchies {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}
