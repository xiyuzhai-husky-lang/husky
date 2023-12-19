pub mod linkage_impl;
pub mod value;

pub use self::linkage_impl::*;
pub use husky_task_prelude_macros::*;

use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;

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

        pub(crate) fn __eager_eval_val_item<T>(
            ingredient_index: usize,
            f: impl FnOnce() -> __Value + 'static,
        ) -> T
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
    base_point: LinkageImpl::Pedestal,
}

impl<LinkageImpl: IsLinkageImpl> Clone for DevEvalContext<LinkageImpl> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime,
            base_point: self.base_point,
        }
    }
}

impl<LinkageImpl: IsLinkageImpl> Copy for DevEvalContext<LinkageImpl> {}

impl<LinkageImpl: IsLinkageImpl> DevEvalContext<LinkageImpl> {
    pub fn new(
        runtime: &'static dyn IsDevRuntimeDyn<LinkageImpl>,
        base_point: LinkageImpl::Pedestal,
    ) -> Self {
        Self {
            runtime,
            base_point,
        }
    }

    pub fn eval_eager_val_item(
        self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        f: impl FnOnce() -> LinkageImpl::Value + 'static,
    ) -> LinkageImpl::Value {
        self.runtime.eval_eager_val_item_dyn(
            jar_index,
            ingredient_index,
            self.base_point,
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
            .eval_lazy_val_item_dyn(jar_index, ingredient_index, self.base_point)
    }

    fn memoized_field(self) -> LinkageImpl::Value {
        todo!()
    }

    pub fn pedestal(&self) -> &LinkageImpl::Pedestal {
        &self.base_point
    }
}

pub trait IsDevRuntime<LinkageImpl: IsLinkageImpl> {
    type StaticSelf: IsDevRuntime<LinkageImpl> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;

    fn eval_eager_val_item(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValueResult<LinkageImpl>,
    ) -> LinkageImpl::Value;

    fn eval_lazy_val_item(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: LinkageImpl::Pedestal,
    ) -> LinkageImpl::Value;
}

pub trait IsDevRuntimeDyn<LinkageImpl: IsLinkageImpl> {
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: LinkageImpl::Pedestal,
        f: Box<dyn FnOnce() -> LinkageImplValueResult<LinkageImpl>>,
    ) -> LinkageImpl::Value;

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: LinkageImpl::Pedestal,
    ) -> LinkageImpl::Value;
}

impl<LinkageImpl: IsLinkageImpl, Runtime> IsDevRuntimeDyn<LinkageImpl> for Runtime
where
    Runtime: IsDevRuntime<LinkageImpl>,
{
    fn eval_eager_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: LinkageImpl::Pedestal,
        f: Box<dyn FnOnce() -> LinkageImplValueResult<LinkageImpl>>,
    ) -> LinkageImpl::Value {
        self.eval_eager_val_item(jar_index, ingredient_index, base_point, f)
    }

    fn eval_lazy_val_item_dyn(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        base_point: <LinkageImpl as IsLinkageImpl>::Pedestal,
    ) -> <LinkageImpl as IsLinkageImpl>::Value {
        self.eval_lazy_val_item(jar_index, ingredient_index, base_point)
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
