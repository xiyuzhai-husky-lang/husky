pub mod value;

pub use husky_task_prelude_macros::*;

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
