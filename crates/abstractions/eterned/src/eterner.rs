use crate::*;
use sealed::sealed;
use std::sync::Mutex;

pub struct Eterner<T: Eq + std::hash::Hash + Send + Sync + 'static> {
    pool: Mutex<Pool<T, 1024>>,
    map: DashMap<T, Eterned<T>>,
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

pub struct EternerDyn(Box<dyn IsEternerDyn>);

impl<T: Eq + std::hash::Hash + Send + Sync + 'static> Eterner<T> {
    pub fn etern(&self, t: T) -> Eterned<T> {
        todo!()
    }

    pub fn etern_ref<Q: Eq + std::hash::Hash + ?Sized>(&self, q: &Q) -> Eterned<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        todo!()
    }
}

impl EternerDyn {
    pub(crate) fn new<T: Eq + std::hash::Hash + Send + Sync + 'static>() -> Self {
        Self(Box::new(Eterner::<T>::default()))
    }

    pub(crate) fn downcast<T: Eq + std::hash::Hash + Send + Sync + 'static>(&self) -> &Eterner<T> {
        (&*self.0 as &dyn std::any::Any)
            .downcast_ref::<Eterner<T>>()
            .unwrap()
    }
}
