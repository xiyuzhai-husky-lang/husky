use husky_atom::*;
use husky_entity_route::EntityRoutePtr;
use singleton::singleton;
use vm::__StaticInfo;

use crate::*;

singleton! {HuskyRuntime}

// pub(crate) static mut HUSKY_EVAL_TIME_SINGLETON: Option<*const HuskyRuntime> = None;

pub fn eval_time() -> &'static HuskyRuntime {
    __access_singleton()
    // unsafe { &*HUSKY_EVAL_TIME_SINGLETON.unwrap() }
}

// pub fn __ty_route_from_static_binded<T: __StaticInfo>(text: &str) -> EntityRoutePtr {
//     compile_time().ty_route_from_static(std::any::TypeId::of::<T::__StaticSelf>(), text)
// }
// pub fn __parse_route_from_text(text: &str) -> EntityRoutePtr {
//     compile_time().parse_route_from_text(text)
// }

pub fn compile_time() -> &'static HuskyComptime {
    eval_time().compile_time()
}
// pub struct HuskyRuntimeSingletonKeeper(Box<HuskyRuntime>);

// impl std::ops::Deref for HuskyRuntimeSingletonKeeper {
//     type Target = HuskyRuntime;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl Drop for HuskyRuntimeSingletonKeeper {
//     fn drop(&mut self) {
//         unsafe { HUSKY_EVAL_TIME_SINGLETON = None }
//     }
// }

// impl From<HuskyRuntime> for HuskyRuntimeSingletonKeeper {
//     fn from(eval_time: HuskyRuntime) -> Self {
//         let boxed_eval_time = Box::new(eval_time);
//         unsafe { HUSKY_EVAL_TIME_SINGLETON = Some(&*boxed_eval_time) };
//         Self(boxed_eval_time)
//     }
// }
