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
fn parse_basic_rose_latex_input_into_asts_works() {
    t(
        "Hello, world!",
        expect![[r#"
            "Hello, world!" all input
            ├─ "Hello" word
            ├─ "," punctuation
            ├─ "world" word
            └─ "!" punctuation
        "#]],
    );
}

#[test]
fn parse_rose_with_math_latex_input_into_asts_works() {
    t(
        "Let $x = 1$.",
        expect![[r#"
            "Let $x = 1$." all input
            ├─ "Let" word
            ├─ "$x = 1$" math
            │ ├─ "x" plain letter
            │ ├─ "=" punctuation
            │ └─ "1" digit
            └─ "." punctuation
        "#]],
    );
}

#[test]
fn paragraph_latex_input_into_asts_works() {
    t(
        r#"Roses are red,
Violets are blue.

Code is my passion,
And testing is too!"#,
        expect![[r#"
            "Roses are red,\nViolets are blue.\n\nCode is my passion,\nAnd testing is too!" all input
            ├─ "Roses" word
            ├─ "are" word
            ├─ "red" word
            ├─ "," punctuation
            ├─ "Violets" word
            ├─ "are" word
            ├─ "blue" word
            ├─ "." punctuation
            ├─ "\n\n" new paragraph
            ├─ "Code" word
            ├─ "is" word
            ├─ "my" word
            ├─ "passion" word
            ├─ "," punctuation
            ├─ "And" word
            ├─ "testing" word
            ├─ "is" word
            ├─ "too" word
            └─ "!" punctuation
        "#]],
    );
}

#[test]
fn parse_rose_equation_environment_into_latex_asts_works() {
    t(
        r#"\begin{equation}x = 1\end{equation}"#,
        expect![[r#"
        "\\begin{equation}x = 1\\end{equation}" all input
        └─ "\\begin{equation}x = 1\\end{equation}" environment
          ├─ "x" plain letter
          ├─ "=" punctuation
          └─ "1" digit
    "#]],
    );
    t(
        "$x-1$",
        expect![[r#"
            "$x-1$" all input
            └─ "$x-1$" math
              ├─ "x" plain letter
              ├─ "-" punctuation
              └─ "1" digit
        "#]],
    );
    t(
        r#"\[x-1\]"#,
        expect![[r#"
            "\\[x-1\\]" all input
            └─ "\\[x-1\\]" math
              ├─ "x" plain letter
              ├─ "-" punctuation
              └─ "1" digit
        "#]],
    );
    t(
        r#"$$x-1$$"#,
        expect![[r#"
            "$$x-1$$" all input
            └─ "$$x-1$$" math
              ├─ "x" plain letter
              ├─ "-" punctuation
              └─ "1" digit
        "#]],
    );
}
