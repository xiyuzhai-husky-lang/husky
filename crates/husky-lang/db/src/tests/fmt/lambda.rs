use super::utils;

#[test]
fn lambda1() {
    utils::test_invariance_under_fmt("|| x");
}

#[test]
fn lambda2() {
    utils::test_invariance_under_fmt("|x| x");
}

#[test]
fn lambda3() {
    utils::test_invariance_under_fmt("|x: i32| x");
}

#[test]
fn lambda4() {
    utils::test_invariance_under_fmt("|x: i32, y: i32| x");
}
