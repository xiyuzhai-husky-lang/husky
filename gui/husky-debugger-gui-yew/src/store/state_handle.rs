use super::*;
use yew::UseStateHandle;

#[derive(Clone)]
pub struct DerivedStateHandle<T>
where
    T: Storable,
{
    pub(super) use_state_handle: UseStateHandle<T>,
    pub(super) store_internal: Rc<Mutex<StoreInternal<T>>>,
}

impl<T> DerivedStateHandle<T> where T: Storable {}

impl<T> std::fmt::Display for DerivedStateHandle<T>
where
    T: Storable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.use_state_handle, f)
    }
}

impl<T> std::fmt::Debug for DerivedStateHandle<T>
where
    T: Storable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&*self.use_state_handle, f)
    }
}

impl<T> DerivedStateHandle<T>
where
    T: Storable,
{
    pub fn set(&self, value: T) {
        log::info!("set");
        self.store_internal.lock().unwrap().set(value)
    }
}

impl<T> std::ops::Deref for DerivedStateHandle<T>
where
    T: Storable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.use_state_handle
    }
}
