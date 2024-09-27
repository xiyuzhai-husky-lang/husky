use super::*;

#[derive(Debug)]
pub struct OwnedThawedValue(Box<dyn ThawedDyn>);

impl std::ops::Deref for OwnedThawedValue {
    type Target = dyn ThawedDyn;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::ops::DerefMut for OwnedThawedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl OwnedThawedValue {
    pub(super) fn index_owned_dyn(self, index: usize) -> ExceptedThawedValue {
        self.0.index_owned_thawed_dyn(index)
    }

    pub(super) fn upcast_from_owned<T>(t: T) -> Self
    where
        T: Thawed,
    {
        Self(Box::<T>::new(t))
    }

    pub fn downcast_into_owned<T>(self) -> T
    where
        T: 'static,
    {
        *((self.0 as Box<dyn std::any::Any>).downcast().unwrap())
    }

    pub(super) fn downcast_as_ref<T>(&self) -> &T
    where
        T: Boiled,
    {
        unsafe {
            std::mem::transmute(
                ((&*self.0 as &dyn ThawedDyn) as &dyn std::any::Any)
                    .downcast_ref::<T::Thawed>()
                    .unwrap(),
            )
        }
    }

    pub(super) fn into_inner(self) -> Box<dyn ThawedDyn> {
        self.0
    }

    pub(super) fn as_ref(&self) -> &dyn ThawedDyn {
        &*self.0
    }

    pub(super) fn freeze(&self) -> Arc<dyn FrozenDyn> {
        self.0.freeze()
    }
}
