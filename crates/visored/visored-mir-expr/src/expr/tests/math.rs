use helpers::tracker::VdMirExprTracker;
use latex_prelude::helper::tracker::LxFormulaInput;

use super::*;

fn t(input: &str, expect: &Expect) {
    let db = &DB::default();
    let tracker = VdMirExprTracker::new(LxFormulaInput(input), &[], &[], db);
    expect.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn basic_to_vd_mir_works() {
    t(
        "1 + 1",
        &expect![[r#"
            separator
            ├─ 1
            └─ 1
        "#]],
    );
    t(
        "1",
        &expect![[r#"
            1
        "#]],
    );
    t(
        "-1",
        &expect![[r#"
            prefix opr
            └─ 1
        "#]],
    );
    t(
        "1<2",
        &expect![[r#"
            separator
            ├─ 1
            └─ 2
        "#]],
    );
}

#[test]
fn frac_to_vd_mir_works() {
    t(
        "\\frac{1}{2}",
        &expect![[r#"
        frac
        ├─ 1
        └─ 2
    "#]],
    );
}

#[test]
fn sqrt_to_vd_mir_works() {
    t(
        "\\sqrt{1}",
        &expect![[r#"
        sqrt
        └─ 1
    "#]],
    );
}
