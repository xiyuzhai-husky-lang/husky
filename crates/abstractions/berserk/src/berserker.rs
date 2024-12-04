use crate::*;
use husky_sha_utils::Sha512Output;
use sealed::sealed;
use std::{pin::Pin, sync::Mutex};

pub struct Berserker<T: Eq + std::hash::Hash + Send + Sync + 'static> {
    pool: Mutex<Pool<BerserkEntry<T>, 1024>>,
    map: DashMap<T, Berserk<'static, T>>,
}

#[derive(Debug)]
pub struct BerserkEntry<T> {
    pub value: T,
    pub id: u32,
    pub sha512: Sha512Output,
}

impl<T: std::hash::Hash> BerserkEntry<T> {
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

impl<T> std::hash::Hash for BerserkEntry<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sha512.hash(state);
    }
}

impl<'db, T: Eq + std::hash::Hash + Send + Sync + 'db> Default for Berserker<T> {
    fn default() -> Self {
        Self {
            pool: Mutex::new(Pool::default()),
            map: DashMap::new(),
        }
    }
}

#[sealed]
pub trait IsBerserkerDyn: std::any::Any + Send + Sync + 'static {
    fn t_type_id(&self) -> std::any::TypeId;
    fn downcast_aux<'db>(&'db self) -> &'db ();
}

#[sealed]
impl<T: Eq + std::hash::Hash + Send + Sync + 'static> IsBerserkerDyn for Berserker<T> {
    fn t_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn downcast_aux<'db>(&'db self) -> &'db () {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'db> dyn IsBerserkerDyn + 'db {
    pub fn downcast_ref<T: Eq + std::hash::Hash + Send + Sync + 'static>(&self) -> &Berserker<T> {
        unsafe { std::mem::transmute(self.downcast_aux()) }
    }
}

pub struct BerserkerDyn(Pin<Box<dyn IsBerserkerDyn>>);

impl<'db, T: Clone + Eq + std::hash::Hash + Send + Sync + 'db> Berserker<T> {
    pub fn with_pool<R>(&self, f: impl FnOnce(&Pool<BerserkEntry<T>, 1024>) -> R) -> R {
        f(&self.pool.lock().unwrap())
    }

    pub fn berserk(&self, t: T) -> Berserk<T> {
        if let Some(berserk) = self.map.get(&t) {
            return *berserk;
        }
        let mut pool = self.pool.lock().unwrap();
        // in racing conditions, another thread might have already allocated the value
        if let Some(berserk) = self.map.get(&t) {
            return *berserk;
        }
        let berserk_entry = BerserkEntry::new(t.clone(), pool.len());
        let berserk = Berserk(unsafe { &*pool.alloc(berserk_entry) });
        self.map.insert(t, berserk);
        berserk
    }

    pub fn berserk_ref<S, Q: Eq + std::hash::Hash + ?Sized>(&self, q: &Q) -> Berserk<S::Static>
    where
        S: AsStatic<Static = T> + std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        if let Some(berserk) = self.map.get(q) {
            return unsafe { std::mem::transmute(*berserk) };
        }
        let mut pool = self.pool.lock().unwrap();
        // in racing conditions, another thread might have already allocated the value
        if let Some(berserk) = self.map.get(q) {
            return unsafe { std::mem::transmute(*berserk) };
        }
        let t: T = q.into();
        let berserk_entry = BerserkEntry::new(t.clone(), pool.len());
        let berserk = Berserk(unsafe { &*pool.alloc(berserk_entry) });
        self.map.insert(t, berserk);
        unsafe { std::mem::transmute(berserk) }
    }
}

impl BerserkerDyn {
    pub(crate) fn new<T: Eq + std::hash::Hash + Send + Sync + 'static>() -> Self {
        Self(Box::pin(Berserker::<T>::default()))
    }

    pub(crate) fn downcast_ref<'db, T>(&'db self) -> &'db Berserker<T>
    where
        T: Eq + std::hash::Hash + Send + Sync,
    {
        self.0.downcast_ref::<T>()
    }
}
