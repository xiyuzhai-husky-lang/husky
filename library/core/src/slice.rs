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
    T: Visualize,
{
    fn visualize(&self, visual_synchrotron: &mut __VisualSynchrotron) -> Visual {
        let elements = self
            .iter()
            .map(|t| t.visualize(visual_synchrotron))
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}

impl<T> __Static for CyclicSliceLeashed<T>
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

    fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        todo!()
    }
}
impl<T> __Frozen for CyclicSliceLeashed<T>
where
    T: __Static,
{
    type Static = Self;
    type Stand = ();
    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
impl<T> __WeakStatic for CyclicSliceLeashed<T>
where
    T: __Static,
{
    type Static = Self;
    unsafe fn into_static(self) -> Self::Static {
        self
    }
}
impl<T> __FromValue for CyclicSliceLeashed<T>
where
    T: __Static,
{
    fn from_value_aux(value: __Value, _: Option<&mut __ValueStands>) -> Self {
        todo!()
    }
}
impl<T> __IntoValue for CyclicSliceLeashed<T>
where
    T: __Static,
{
    fn into_value(self) -> __Value {
        todo!()
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

impl<T> CyclicSliceLeashed<T> {
    pub fn new(slice: &'static [T], start: i32, end: i32) -> Self {
        Self { start, end, slice }
    }

    pub fn first(self) -> Option<&'static T> {
        if self.slice.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(self.index_i32(self.start))
        }
    }

    pub fn last(self) -> Option<&'static T> {
        if self.slice.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(self.index_i32((self.end - 1)))
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

impl<T> std::ops::Index<usize> for CyclicSliceLeashed<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.index_i32(index as i32)
    }
}
