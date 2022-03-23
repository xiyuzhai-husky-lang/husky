use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Row(pub u32);

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

impl From<u32> for Row {
    fn from(raw: u32) -> Self {
        Row(raw)
    }
}

impl From<usize> for Row {
    fn from(raw: usize) -> Self {
        Row(<usize as TryInto<u32>>::try_into(raw).expect("success"))
    }
}

impl From<i32> for Row {
    fn from(raw: i32) -> Self {
        assert!(raw >= 0);
        Row(raw as u32)
    }
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32((self.0 + 1) as i32)
    }
}

impl<'de> Deserialize<'de> for Row {
    fn deserialize<D>(_: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
