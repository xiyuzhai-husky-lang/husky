#[test]
fn overflow() {
    let a: u8 = 100000u32.try_into().unwrap();
    let a: u8 = 100000u32.try_into().unwrap();
}
