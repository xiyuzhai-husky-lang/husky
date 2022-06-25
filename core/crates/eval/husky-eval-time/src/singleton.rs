use crate::*;

pub(crate) static mut HUSKY_EVAL_TIME_SINGLETON: Option<*const HuskyEvalTime> = None;

pub fn husky_eval_time() -> &'static HuskyEvalTime {
    unsafe { &*HUSKY_EVAL_TIME_SINGLETON.unwrap() }
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
