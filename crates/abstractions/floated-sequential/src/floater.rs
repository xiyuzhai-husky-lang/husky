use crate::{Floated, *};
use husky_sha_utils::Sha512Output;
use pool::Pool;
use rustc_hash::FxHashMap;
use sealed::sealed;
use std::{cell::RefCell, pin::Pin, sync::Mutex};

pub struct Floater<T: Eq + std::hash::Hash + 'static> {
    pool: RefCell<Pool<FloatedEntry<T>, 1024>>,
    map: RefCell<FxHashMap<T, Floated<'static, T>>>,
}

#[derive(Debug)]
pub struct FloatedEntry<T> {
    pub value: T,
    pub id: u32,
    pub sha512: Sha512Output,
}

impl<T: std::hash::Hash> FloatedEntry<T> {
    pub fn new(value: T, id: usize) -> Self {
        use husky_sha_utils::ShaHash;

        let sha512 = value.sha512();
        Self {
            value,
            id: id.try_into().unwrap(),
            sha512,
        }
    }
}

impl<T> std::hash::Hash for FloatedEntry<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha512.hash(state);
    }
}

impl<T: Eq + std::hash::Hash + 'static> Default for Floater<T> {
    fn default() -> Self {
        Self {
            pool: RefCell::new(Pool::default()),
            map: RefCell::new(FxHashMap::default()),
        }
    }
}

#[sealed]
pub trait IsFloaterDyn: std::any::Any + 'static {}

#[sealed]
impl<T: Eq + std::hash::Hash + 'static> IsFloaterDyn for Floater<T> {}

pub struct FloaterDyn(Pin<Box<dyn IsFloaterDyn>>);

impl<T: Clone + Eq + std::hash::Hash + 'static> Floater<T> {
    pub fn with_pool<R>(&self, f: impl FnOnce(&Pool<FloatedEntry<T>, 1024>) -> R) -> R {
        f(&self.pool.borrow())
    }

    pub fn float(&self, t: T) -> Floated<T> {
        if let Some(floated) = self.map.borrow().get(&t) {
            return *floated;
        }
        // in racing conditions, another thread might have already allocated the value
        if let Some(floated) = self.map.borrow().get(&t) {
            return *floated;
        }
        let floated_entry = FloatedEntry::new(t.clone(), self.pool.borrow().len());
        let floated = Floated(unsafe { &*self.pool.borrow_mut().alloc(floated_entry) });
        self.map.borrow_mut().insert(t, floated);
        floated
    }

    pub fn float_ref<Q: Eq + std::hash::Hash + ?Sized>(&self, q: &Q) -> Floated<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        if let Some(floated) = self.map.borrow().get(q) {
            return *floated;
        }
        // in racing conditions, another thread might have already allocated the value
        if let Some(floated) = self.map.borrow().get(q) {
            return *floated;
        }
        let t: T = q.into();
        let floated_entry = FloatedEntry::new(t.clone(), self.pool.borrow().len());
        let floated = Floated(unsafe { &*self.pool.borrow_mut().alloc(floated_entry) });
        self.map.borrow_mut().insert(t, floated);
        floated
    }
}

impl FloaterDyn {
    pub(crate) fn new<T: Eq + std::hash::Hash + 'static>() -> Self {
        Self(Box::pin(Floater::<T>::default()))
    }

    pub(crate) fn downcast<T: Eq + std::hash::Hash + 'static>(&self) -> &Floater<T> {
        (&*self.0 as &dyn std::any::Any)
            .downcast_ref::<Floater<T>>()
            .unwrap()
    }
}
