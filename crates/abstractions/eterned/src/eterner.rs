use crate::*;
use husky_sha_utils::Sha512Output;
use sealed::sealed;
use std::{pin::Pin, sync::Mutex};

pub struct Eterner<T: Eq + std::hash::Hash + Send + Sync + 'static> {
    pool: Mutex<Pool<EternedEntry<T>, 1024>>,
    map: DashMap<T, Eterned<T>>,
}

#[derive(Debug)]
pub struct EternedEntry<T> {
    pub value: T,
    pub id: u32,
    pub sha512: Sha512Output,
}

impl<T: std::hash::Hash> EternedEntry<T> {
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

impl<T> std::hash::Hash for EternedEntry<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha512.hash(state);
    }
}

impl<T: Eq + std::hash::Hash + Send + Sync + 'static> Default for Eterner<T> {
    fn default() -> Self {
        Self {
            pool: Mutex::new(Pool::default()),
            map: DashMap::new(),
        }
    }
}

#[sealed]
pub trait IsEternerDyn: std::any::Any + Send + Sync + 'static {}

#[sealed]
impl<T: Eq + std::hash::Hash + Send + Sync + 'static> IsEternerDyn for Eterner<T> {}

pub struct EternerDyn(Pin<Box<dyn IsEternerDyn>>);

impl<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static> Eterner<T> {
    pub fn etern(&self, t: T) -> Eterned<T> {
        if let Some(eterned) = self.map.get(&t) {
            return *eterned;
        }
        let mut pool = self.pool.lock().unwrap();
        // in racing conditions, another thread might have already allocated the value
        if let Some(eterned) = self.map.get(&t) {
            return *eterned;
        }
        let eterned_entry = EternedEntry::new(t.clone(), pool.len());
        let eterned = Eterned(unsafe { &*pool.alloc(eterned_entry) });
        self.map.insert(t, eterned);
        eterned
    }

    pub fn etern_ref<Q: Eq + std::hash::Hash + ?Sized>(&self, q: &Q) -> Eterned<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        if let Some(eterned) = self.map.get(q) {
            return *eterned;
        }
        let mut pool = self.pool.lock().unwrap();
        // in racing conditions, another thread might have already allocated the value
        if let Some(eterned) = self.map.get(q) {
            return *eterned;
        }
        let t: T = q.into();
        let eterned_entry = EternedEntry::new(t.clone(), pool.len());
        let eterned = Eterned(unsafe { &*pool.alloc(eterned_entry) });
        self.map.insert(t, eterned);
        eterned
    }
}

impl EternerDyn {
    pub(crate) fn new<T: Eq + std::hash::Hash + Send + Sync + 'static>() -> Self {
        Self(Box::pin(Eterner::<T>::default()))
    }

    pub(crate) fn downcast<T: Eq + std::hash::Hash + Send + Sync + 'static>(&self) -> &Eterner<T> {
        (&*self.0 as &dyn std::any::Any)
            .downcast_ref::<Eterner<T>>()
            .unwrap()
    }
}
