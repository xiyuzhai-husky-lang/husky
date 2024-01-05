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
            Some(self.index(self.start as usize))
        }
    }

    pub fn last(self) -> Option<&'static T> {
        if self.slice.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(self.index((self.end - 1) as usize))
        }
    }

    pub fn start(self) -> i32 {
        self.start
    }

    pub fn end(self) -> i32 {
        self.end
    }

    pub fn index(&self, index: usize) -> &'static T {
        &self.slice[index.rem_euclid(self.slice.len())]
    }
}

impl<T> std::ops::Index<usize> for CyclicSliceLeashed<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.index(index)
    }
}
