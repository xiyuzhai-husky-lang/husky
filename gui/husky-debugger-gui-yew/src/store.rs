mod internal;
mod state_handle;
mod storable;

use internal::*;
use state_handle::*;
use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Mutex,
};
use storable::*;
use yew::{use_state, UseStateHandle};

#[derive(Clone)]
pub struct Store<T>
where
    T: Storable,
{
    internal: Rc<Mutex<StoreInternal<T>>>,
}

impl<T> std::fmt::Debug for Store<T>
where
    T: Storable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Store<T>
where
    T: Storable,
{
    pub fn new(value: T) -> Self {
        Store {
            internal: Rc::new(Mutex::new(StoreInternal::new(value))),
        }
    }

    pub fn subscribe(&self, subscriber: Box<dyn Fn(T)>) -> Box<dyn FnOnce()> {
        let id = self.internal.lock().unwrap().subscribe(subscriber);
        {
            let internal = self.internal.clone();
            Box::new(move || internal.lock().unwrap().unsubscribe(id))
        }
    }

    pub fn value(&self) -> T {
        self.internal.lock().unwrap().value()
    }

    pub fn set(&self, value: T) {
        self.internal.lock().unwrap().set(value)
    }
}

pub fn use_store(store: Store<i32>) -> DerivedStateHandle<i32> {
    let use_state_handle = use_state(|| store.value());
    store.set(1);
    let subscriber = {
        let use_state_handle = use_state_handle.clone();
        Box::new(move |value| use_state_handle.set(value))
    };
    store.subscribe(subscriber);
    DerivedStateHandle {
        use_state_handle,
        store_internal: store.internal.clone(),
    }
}
