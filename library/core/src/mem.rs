use crate::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq)]
pub struct Leash<T>(pub &'static T)
where
    T: ?Sized + 'static;

impl<T> Serialize for Leash<T>
where
    T: Serialize + ?Sized + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> Clone for Leash<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for Leash<T> {}

impl<T> std::hash::Hash for Leash<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

impl<T> Leash<T>
where
    T: ?Sized + 'static,
{
    pub fn new(t: &'static T) -> Self {
        Self(t)
    }
}

impl<T> Leash<T>
where
    T: ?Sized + 'static,
{
    pub fn deleash(self) -> &'static T {
        self.0
    }
}

// ad hoc, should use Leash<[T]>
impl<T> Leash<Vec<T>>
where
    T: 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

impl<T> Leash<[T]>
where
    T: 'static,
{
    pub fn collect_leashes(self) -> Vec<Leash<T>> {
        self.0.iter().map(Leash).collect()
    }
}

impl<T> __Static for Leash<T>
where
    T: __Static,
{
    type Frozen = Self;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> __JsonValue {
        todo!("CyclicSlice serialize_to_value")
    }

    fn visualize_or_void(&self, _visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        todo!()
    }

    fn unwrap_option_ref<'a>(__self: &'a Option<Self>) -> __Value {
        todo!()
    }

    fn unwrap_option_leash(__self: &'static Option<Self>) -> __Value {
        todo!("type_name = `{}`", std::any::type_name::<T>())
    }
}
impl<T> __Frozen for Leash<T>
where
    T: __Static,
{
    type Static = Self;
    type Stand = ();
    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
impl<T> __WeakStatic for Leash<T>
where
    T: __Static,
{
    type Static = Self;
    unsafe fn into_static(self) -> Self::Static {
        self
    }
}
impl<T> __FromValue for Leash<T>
where
    T: __Static,
{
    fn from_value_aux(_value: __Value, _: Option<&mut __ValueStands>) -> Self {
        Leash(_value.into_leash())
    }
}

impl<T> __IntoValue for Leash<T>
where
    T: __Static,
{
    fn into_value(self) -> __Value {
        __Value::from_leash(self.deleash())
    }
}

impl<T> __FromValue for Leash<[T]>
where
    T: __Static,
{
    fn from_value_aux(value: __Value, value_stands: Option<&mut __ValueStands>) -> Self {
        todo!()
    }
}

impl<T> __IntoValue for Leash<[T]>
where
    T: __Static,
{
    fn into_value(self) -> __Value {
        todo!()
    }
}
