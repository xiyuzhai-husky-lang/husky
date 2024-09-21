use super::*;
use crate::thawed::Thawed;
use thawed::r#ref::ThawedRef;

#[derive(Debug)]
pub struct FrozenRef<T>(Arc<T>)
where
    T: Frozen;

impl<T> FrozenRef<T>
where
    T: Frozen,
{
    pub(crate) unsafe fn new(t: *mut T) -> Self {
        todo!()
        // Self(Box::new(<T as Thawed>::freeze(&*t)))
    }
}

impl<T> Clone for FrozenRef<T>
where
    T: Frozen,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Frozen for FrozenRef<T>
where
    T: Frozen,
{
    type Thawed = ThawedRef<T::Thawed>;

    type Slush = Box<T::Slush>;

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
