use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::*;

// #[husky_standard_value::value_conversion]
#[derive(Debug, PartialEq, Eq, serde::Serialize)]
#[serde(crate = "self::serde")]
pub struct CyclicSliceLeashed<T>
where
    T: 'static,
{
    slice: &'static [T],
    start: i32,
    end: i32,
}

impl<T> Visualize for CyclicSliceLeashed<T>
where
    T: Visualize + __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> Visual {
        let elements = self
            .iter()
            .map(|t| t.visualize(visual_synchrotron))
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}

impl<T> __Immortal for CyclicSliceLeashed<T>
where
    T: __Immortal + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn try_copy(&self) -> Option<__Value> {
        todo!()
    }

    fn index_owned(self: Self, index: usize) -> __ExceptedValue {
        Ok(Leash(self.index_i32(index.try_into().unwrap())).into_value())
    }
}

impl<T> __Thawed for CyclicSliceLeashed<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Frozen = Self;

    fn is_copyable() -> bool {
        todo!()
    }

    fn try_copy_thawed(&self) -> Option<__ThawedValue> {
        todo!()
    }

    fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn index_owned_thawed(self: Self, index: usize) -> __ExceptedThawedValue {
        Ok(Leash(self.index_i32(index.try_into().unwrap())).into_thawed_value())
    }

    fn index_ref_thawed<'a>(&'a self, index: usize) -> __ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support indexing ref",
            std::any::type_name::<Self>()
        )
    }

    fn index_leash_thawed(&'static self, index: usize) -> __ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support indexing leash",
            std::any::type_name::<Self>()
        )
    }
}

impl<T> __Frozen for CyclicSliceLeashed<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Thawed = Self;
    type Slush = ();
    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }

    fn serialize_to_value(&self) -> __JsonValue {
        todo!("CyclicSlice serialize_to_value")
    }

    fn visualize_or_void(&self, _visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        todo!()
    }
}

impl<T> __Boiled for CyclicSliceLeashed<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Thawed = Self;

    unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
        std::mem::transmute(thawed)
    }

    #[inline]
    unsafe fn from_thawed_ref(thawed_ref: &Self::Thawed) -> &Self {
        std::mem::transmute(thawed_ref)
    }

    unsafe fn into_thawed(self) -> Self::Thawed {
        self
    }
}

impl<T> __FromValue for CyclicSliceLeashed<T>
where
    T: __Immortal,
{
    fn from_value_aux(value: __Value, _: Option<&mut __SlushValues>) -> Self {
        value.into_owned()
    }
}
impl<T> __IntoValue for CyclicSliceLeashed<T>
where
    T: __Immortal + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn into_value(self) -> __Value {
        __Value::from_owned(self)
    }
}

impl<T> __FromThawedValue for CyclicSliceLeashed<T>
where
    T: __Thawed,
{
    fn from_thawed_value_aux(value: __ThawedValue, _: Option<&mut __SlushValues>) -> Self {
        value.into_owned()
    }
}

impl<T> __IntoThawedValue for CyclicSliceLeashed<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    fn into_thawed_value(self) -> __ThawedValue {
        __ThawedValue::from_owned(self)
    }
}

impl<T> Clone for CyclicSliceLeashed<T> {
    fn clone(&self) -> Self {
        Self {
            slice: self.slice,
            start: self.start,
            end: self.end,
        }
    }
}

impl<T> Copy for CyclicSliceLeashed<T> {}

impl<T> CyclicSliceLeashed<T>
where
    T: std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    pub fn new(slice: &'static [T], start: i32, end: i32) -> Self {
        Self { start, end, slice }
    }

    pub fn deleash(self) -> Self {
        self
    }

    pub fn first(self) -> Option<Leash<T>> {
        if self.slice.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(Leash(self.index_i32(self.start)))
        }
    }

    pub fn last(self) -> Option<Leash<T>> {
        if self.slice.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(Leash(self.index_i32(self.end - 1)))
        }
    }

    pub fn start(self) -> i32 {
        self.start
    }

    pub fn end(self) -> i32 {
        self.end
    }

    pub fn index_i32(self, index: i32) -> &'static T {
        let rem_euclid = index.rem_euclid(self.slice.len() as i32);
        debug_assert!(rem_euclid >= 0);
        &self.slice[rem_euclid as usize]
    }

    fn iter(self) -> impl Iterator<Item = &'static T> {
        (self.start..self.end)
            .into_iter()
            .map(move |i| self.index_i32(i as i32))
    }
}

impl<T> CyclicSliceLeashed<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    pub fn index(&self, index: usize) -> &'static T {
        &self.index_i32(index as i32)
    }
}
