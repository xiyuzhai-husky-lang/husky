use super::*;
use crate::test_helpers::example::LxAstExample;
use expect_test::Expect;

fn t(input: &str, expected: Expect) {
    let db = &DB::default();
    let example = LxAstExample::new(input, LxMode::Rose, db);
    let show = example.show(db);
    expected.assert_eq(&show);
}

#[test]
fn parse_basic_math_latex_input_into_asts_then_show_works() {
    t(
        "Hello, world!",
        expect![[r#"
            Hello, world!
            ├─ Hello
            ├─ ,
            ├─ world
            └─ !
        "#]],
    );
}
