use super::*;
use sealed::sealed;
use std::sync::Mutex;

pub struct Jar0<T> {
    pub data: Mutex<Option<Pin<Box<T>>>>,
}

impl<T> Jar0<T> {
    #[inline(always)]
    pub fn get_or_alloc(&self, _key: (), f: impl FnOnce() -> T) -> &T {
        use husky_wild_utils::arb_ref;

        let mut lock = self.data.lock().unwrap();
        if let Some(t) = lock.as_ref() {
            return unsafe { arb_ref(&**t) };
        }
        let t = f();
        let pin = Box::pin(t);
        let t = unsafe { arb_ref(&*pin) };
        *lock = Some(pin);
        t
    }
}

impl<T> Default for Jar0<T> {
    fn default() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }
}

#[sealed]
impl<T> IsMemoJar for Jar0<T> where T: Send + Sync + 'static {}

#[sealed]
impl<T> IsMemoJarDyn for Jar0<T> where T: Send + Sync + 'static {}
