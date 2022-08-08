#[test]
fn test_option_residual() {
    assert_eq!(f(), None);

    fn f() -> Option<i32> {
        let a: Option<i32> = None;
        let b: Option<i32> = Some(6);
        b?;
        a?;
        None
    }
}
