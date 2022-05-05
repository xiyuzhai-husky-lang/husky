use serde::{Deserialize, Serialize};
use test_utils::{TestComparable, TestCompareConfig};

#[derive(
    Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub struct Column(pub(crate) u32);

impl TestComparable for Column {
    fn write_inherent(&self, config: TestCompareConfig, result: &mut String) {
        todo!()
    }
}

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
    let _b: u32 = a as u32;
}
