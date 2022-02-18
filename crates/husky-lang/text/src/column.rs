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
