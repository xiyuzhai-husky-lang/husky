pub mod value;

pub use husky_task_prelude_macros::*;

use once_cell::sync::OnceCell;
use shifted_unsigned_int::ShiftedU32;

#[macro_export]
macro_rules! init_crate {
    () => {
        pub(crate) static __JAR_INDEX: __JarIndexOnceCell = __JarIndexOnceCell::new();

        pub fn __set_jar_index(jar_index: __JarIndex) {
            __JAR_INDEX.set(jar_index).unwrap();
        }

        pub(crate) fn __get_jar_index(jar_index: __JarIndex) {
            __JAR_INDEX.get().expect("`__JAR_INDEX` is not initialized");
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JarIndex(ShiftedU32);

pub type JarIndexOnceCell = OnceCell<JarIndex>;

#[derive(Clone, Copy)]
pub struct DevEvalContext<BasePoint: 'static> {
    runtime: &'static dyn IsDevRuntimeDyn<BasePoint>,
    base_point: BasePoint,
}

impl<BasePoint: 'static> DevEvalContext<BasePoint> {
    pub fn new(runtime: &'static dyn IsDevRuntimeDyn<BasePoint>, base_point: BasePoint) -> Self {
        Self {
            runtime,
            base_point,
        }
    }

    pub fn eval_val_item<T>(self, f: impl FnOnce() -> T) -> T {
        todo!()
    }

    pub fn eval_val_item_return_ref<T>(self, f: impl FnOnce() -> T) -> &'static T {
        todo!()
    }

    fn memoized_field<T>(self) -> T {
        todo!()
    }

    pub fn base_point(&self) -> &BasePoint {
        &self.base_point
    }
}

pub trait IsDevRuntime<BasePoint> {
    type StaticSelf: IsDevRuntime<BasePoint> + 'static;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf;
}

pub trait IsDevRuntimeDyn<BasePoint> {}

impl<BasePoint, Runtime> IsDevRuntimeDyn<BasePoint> for Runtime where
    Runtime: IsDevRuntime<BasePoint>
{
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
