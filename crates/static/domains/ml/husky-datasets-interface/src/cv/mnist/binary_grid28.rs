use super::*;
#[cfg(feature = "serde_support")]
use serde::Serialize;
use std::any::TypeId;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28 {
    padded_rows: [u32; 31],
}

#[cfg(feature = "any_support")]
impl<'eval> __WithEvalLifetime<'eval> for BinaryGrid28 {
    type This = BinaryGrid28;
}

#[cfg(feature = "any_support")]
impl __Any for BinaryGrid28 {}

impl std::ops::Index<usize> for BinaryGrid28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.padded_rows[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryGrid28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.padded_rows[index]
    }
}

impl std::fmt::Debug for BinaryGrid28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryGrid {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryGrid28 {
    pub fn __call__() -> Self {
        Self {
            padded_rows: Default::default(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<u32> {
        self.padded_rows.get(index).map(|x| *x)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.padded_rows.get_mut(index)
    }
}

#[cfg(feature = "serde_support")]
impl Serialize for BinaryGrid28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl __StaticInfo for BinaryGrid28 {
    type __StaticSelf = Self;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __Registrable<'eval> for BinaryGrid28 {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
