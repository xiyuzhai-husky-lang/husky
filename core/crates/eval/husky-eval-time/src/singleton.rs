use entity_route::EntityRoutePtr;
use husky_atom::*;

use crate::*;

pub(crate) static mut HUSKY_EVAL_TIME_SINGLETON: Option<*const HuskyEvalTime> = None;

pub fn eval_time() -> &'static HuskyEvalTime {
    unsafe { &*HUSKY_EVAL_TIME_SINGLETON.unwrap() }
}

pub fn parse_entity_route(text: &str) -> EntityRoutePtr {
    let eval_time = eval_time();
    let mut context = AtomContextStandalone {
        opt_package_main: Some(eval_time.package_main),
        db: eval_time.compile_time(),
        opt_this_ty: None,
        opt_this_contract: None,
        symbols: (&[] as &[Symbol]).into(),
        kind: AtomContextKind::Normal,
    };
    context.parse_entity_route(text).unwrap()
}

pub fn compile_time() -> &'static HuskyCompileTime {
    eval_time().compile_time()
}

pub struct HuskyEvalTimeSingleton(Box<HuskyEvalTime>);

impl std::ops::Deref for HuskyEvalTimeSingleton {
    type Target = HuskyEvalTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for HuskyEvalTimeSingleton {
    fn drop(&mut self) {
        unsafe { HUSKY_EVAL_TIME_SINGLETON = None }
    }
}

impl From<HuskyEvalTime> for HuskyEvalTimeSingleton {
    fn from(eval_time: HuskyEvalTime) -> Self {
        let boxed_eval_time = Box::new(eval_time);
        unsafe { HUSKY_EVAL_TIME_SINGLETON = Some(&*boxed_eval_time) };
        Self(boxed_eval_time)
    }
}
