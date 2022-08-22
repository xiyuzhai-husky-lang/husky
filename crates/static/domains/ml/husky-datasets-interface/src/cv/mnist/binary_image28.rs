use super::*;
use std::{any::TypeId, borrow::Cow, sync::Arc};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28 {
    pub padded_rows: [u32; 30],
}

#[cfg(feature = "any_support")]
impl<'eval> __WithEvalLifetime<'eval> for BinaryImage28 {
    type __ThisWithEvalLifetime = BinaryGrid28;
}

#[cfg(feature = "any_support")]
impl __Any for BinaryImage28 {}

impl std::ops::Index<usize> for BinaryImage28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.padded_rows[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryImage28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.padded_rows[index]
    }
}

impl std::fmt::Debug for BinaryImage28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryImage28 {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryImage28 {
    pub fn __call__() -> Self {
        Self {
            padded_rows: Default::default(),
        }
    }

    pub fn read(content: &[u8]) -> Self {
        assert_eq!(content.len(), 28 * 4);
        let mut padded_rows = [0; 30];
        for i in 0..28 {
            let mut row = 0u32;
            for k in 0..4 {
                row |= (content[i * 4 + k] as u32) << (3 - k) * 8;
            }
            padded_rows[i + 1] = row;
        }
        Self { padded_rows }
    }

    pub(crate) fn get(&self, index: usize) -> Option<u32> {
        self.padded_rows.get(index).map(|x| *x)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.padded_rows.get_mut(index)
    }
}

#[cfg(feature = "serde_support")]
impl Serialize for BinaryImage28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl __StaticInfo for BinaryImage28 {
    type __StaticSelf = Self;

    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }

    unsafe fn __as_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl<'eval> __Registrable<'eval> for BinaryImage28 {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
