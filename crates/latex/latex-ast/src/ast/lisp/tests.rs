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
fn parse_literal_lisp_latex_input_into_asts_works() {
    t(
        "0",
        expect![[r#"
            "0" all input
            └─ "0" literal
        "#]],
    );
    t(
        "0.0",
        expect![[r#"
            "0.0" all input
            └─ "0.0" literal
        "#]],
    );
    t(
        "\"hilbert nullstellensatz\"",
        expect![[r#"
            "\"hilbert nullstellensatz\"" all input
            └─ "\"hilbert nullstellensatz\"" literal
        "#]],
    );
    t(
        "\"\\\"\"",
        expect![[r#"
            "\"\\\"\"" all input
            └─ "\"\\\"\"" literal
        "#]],
    );
    t(
        "\"\\\\\"",
        expect![[r#"
            "\"\\\\\"" all input
            └─ "\"\\\\\"" literal
        "#]],
    );
}

#[test]
fn parse_ident_lisp_latex_input_into_asts_works() {
    t(
        "x",
        expect![[r#"
            "x" all input
            └─ "x" ident
        "#]],
    );
    t(
        "x_1",
        expect![[r#"
            "x_1" all input
            └─ "x_1" ident
        "#]],
    );
    t(
        "apply",
        expect![[r#"
            "apply" all input
            └─ "apply" ident
        "#]],
    );
    t(
        "apply_to_all",
        expect![[r#"
            "apply_to_all" all input
            └─ "apply_to_all" ident
        "#]],
    );
}

#[test]
fn parse_xlabel_lisp_latex_input_into_asts_works() {
    t(
        "'1",
        expect![[r#"
            "'1" all input
            └─ "'1" xlabel
        "#]],
    );
    t(
        "'x",
        expect![[r#"
            "'x" all input
            └─ "'x" xlabel
        "#]],
    );
    t(
        "'eq:1",
        expect![[r#"
            "'eq:1" all input
            └─ "'eq:1" xlabel
        "#]],
    );
}
#[test]
fn parse_parenthesized_lisp_latex_input_into_asts_works() {
    t(
        "(x)",
        expect![[r#"
            "(x)" all input
            └─ "(x)" parenthesized
              └─ "x" ident
        "#]],
    );
    t(
        "f (g x) y",
        expect![[r#"
            "f (g x) y" all input
            ├─ "f" ident
            ├─ "(g x)" parenthesized
            │ ├─ "g" ident
            │ └─ "x" ident
            └─ "y" ident
        "#]],
    );
}

#[test]
fn parse_basic_lisp_latex_input_into_asts_works() {
    t(
        "x",
        expect![[r#"
            "x" all input
            └─ "x" ident
        "#]],
    );
    t(
        "f x y",
        expect![[r#"
            "f x y" all input
            ├─ "f" ident
            ├─ "x" ident
            └─ "y" ident
        "#]],
    );
}

#[test]
fn parse_boxed_lisp_latex_input_into_asts_works() {
    t(
        "[x]",
        expect![[r#"
            "[x]" all input
            └─ "[x]" boxed list
              └─ item
                └─ "x" ident
        "#]],
    );
    t(
        "[x, y]",
        expect![[r#"
            "[x, y]" all input
            └─ "[x, y]" boxed list
              ├─ item
              │ └─ "x" ident
              └─ item
                └─ "y" ident
        "#]],
    );
}
