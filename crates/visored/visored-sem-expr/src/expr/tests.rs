use super::*;
use crate::test_helpers::example::VdSemExprExample;
use latex_prelude::mode::LxMode;

#[test]
fn basic_vd_sem_expr_works() {
    let db = &DB::default();
    let example = VdSemExprExample::new("1+1", LxMode::Math, &[], &[], db);
}
