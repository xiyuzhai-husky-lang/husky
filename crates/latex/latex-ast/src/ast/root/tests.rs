use super::*;
use crate::test_helpers::example::LxAstExample;
use expect_test::{expect, Expect};
use latex_prelude::mode::LxMode;

fn t(input: &str, expected: Expect) {
    let db = &DB::default();
    let example = LxAstExample::new(input, LxMode::Root, db);
    let show = example.show(db);
    expected.assert_eq(&show);
}

#[test]
fn parse_use_packages_into_lx_root_asts_works() {
    t(
        "\\usepackage{amsmath}",
        expect![[r#"
            "\\usepackage{amsmath}" all input
            └─ "\\usepackage{amsmath}" complete command
              └─ amsmath
        "#]],
    );
}
