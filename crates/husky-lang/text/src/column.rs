use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Column(pub(crate) u32);

impl From<u32> for Column {
    fn from(raw: u32) -> Self {
        Column(raw)
    }
}

impl From<usize> for Column {
    fn from(raw: usize) -> Self {
        Column(<usize as TryInto<u32>>::try_into(raw).expect("success"))
    }
}

impl From<i32> for Column {
    fn from(raw: i32) -> Self {
        assert!(raw >= 0);
        Column(raw as u32)
    }
}

#[test]
fn test_conversion() {
    let a: i32 = -1;
    let b: u32 = a as u32;
}

impl Serialize for Column {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32((self.0 + 1) as i32)
    }
}

impl<'de> Deserialize<'de> for Column {
    fn deserialize<D>(_: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
