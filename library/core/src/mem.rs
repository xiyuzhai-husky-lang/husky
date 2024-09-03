use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq)]
pub struct Leash<T>(pub &'static T)
where
    T: ?Sized + Send + Sync + 'static;

impl<T> Serialize for Leash<T>
where
    T: Serialize + ?Sized + Send + Sync + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> Clone for Leash<T>
where
    T: ?Sized + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for Leash<T> where T: ?Sized + Send + Sync + 'static {}

impl<T> std::hash::Hash for Leash<T>
where
    T: ?Sized + Send + Sync + 'static,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

impl<T> Leash<T>
where
    T: ?Sized + Send + Sync + 'static,
{
    pub fn new(t: &'static T) -> Self {
        Self(t)
    }
}

impl<T> Leash<T>
where
    T: ?Sized + Send + Sync + 'static,
{
    pub fn deleash(self) -> &'static T {
        self.0
    }
}

// ad hoc, should use Leash<[T]>
impl<T> Leash<Vec<T>>
where
    T: Send + Sync + 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

impl<T> Leash<[T]>
where
    T: Send + Sync + 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

impl<T> __Thawed for Leash<T>
where
    T: __Thawed + std::fmt::Debug + ?Sized + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Frozen = Self;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn is_copyable() -> bool {
        true
    }

    fn try_copy(&self) -> Option<__ThawedValue> {
        Some((*self).into_thawed_value())
    }

    fn serialize_to_value(&self) -> __JsonValue {
        todo!("CyclicSlice serialize_to_value")
    }

    fn visualize_or_void(&self, _visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        todo!()
    }
}

impl<T> __Frozen for Leash<T>
where
    T: __Thawed + std::fmt::Debug + ?Sized + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Thawed = Self;
    type Slush = ();
    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }
}
impl<T> __Boiled for Leash<T>
where
    T: __Thawed + std::fmt::Debug + ?Sized + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Thawed = Self;
    unsafe fn into_thawed(self) -> Self::Thawed {
        self
    }
}

impl<T> __FromValue for Leash<T>
where
    T: __Immortal + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn from_value_aux(_value: __Value, _: Option<&mut __SlushValues>) -> Self {
        Leash(_value.into_leash())
    }
}

impl<T> __IntoValue for Leash<T>
where
    T: __Immortal + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn into_value(self) -> __Value {
        __Value::from_leash(self.deleash())
    }
}

impl<T> __FromThawedValue for Leash<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn from_thawed_value_aux(thawed_value: __ThawedValue, _: Option<&mut __SlushValues>) -> Self {
        Leash(thawed_value.into_leash())
    }
}

impl<T> __IntoThawedValue for Leash<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn into_thawed_value(self) -> __ThawedValue {
        __ThawedValue::from_leash(self.deleash())
    }
}
impl<T> __FromValue for Leash<[T]>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn from_value_aux(value: __Value, slush_values: Option<&mut __SlushValues>) -> Self {
        todo!()
    }
}

impl<T> __IntoValue for Leash<[T]>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn into_value(self) -> __Value {
        todo!()
    }
}
