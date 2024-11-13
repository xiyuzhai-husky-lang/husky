use super::*;
use crate::test_helpers::example::LxAstExample;
use expect_test::Expect;

fn t(input: &str, expected: Expect) {
    let db = &DB::default();
    let example = LxAstExample::new(input, LxMode::Lisp, db);
    let show = example.show(db);
    expected.assert_eq(&show);
}

#[test]
fn parse_basic_lisp_latex_input_into_asts_works() {
    t(
        "x",
        expect![[r#"
            x
            └─ x
        "#]],
    );
    t(
        "f x y",
        expect![[r#"
            f x y
            ├─ f
            ├─ x
            └─ y
        "#]],
    );
}

#[test]
fn parse_parenthesized_lisp_latex_input_into_asts_works() {
    t(
        "(x)",
        expect![[r#"
            (x)
            └─ (x)
              └─ x
        "#]],
    );
    t(
        "f (g x) y",
        expect![[r#"
            f (g x) y
            ├─ f
            ├─ (g x)
            │ ├─ g
            │ └─ x
            └─ y
        "#]],
    );
}

#[test]
fn parse_boxed_lisp_latex_input_into_asts_works() {
    t(
        "[x]",
        expect![[r#"
            [x]
            └─ [x]
              └─ item
                └─ x
        "#]],
    );
    t(
        "[x, y]",
        expect![[r#"
            [x, y]
            └─ [x, y]
              ├─ item
              │ └─ x
              └─ item
                └─ y
        "#]],
    );
}
