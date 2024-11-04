use super::*;
use crate::test_helpers::example::VdSynExprExample;
use expect_test::{expect, Expect};
use latex_prelude::mode::LxMode;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

fn t(
    input: &str,
    token_annotations: &[((&str, &str), VdTokenAnnotation)],
    space_annotations: &[((&str, &str), VdSpaceAnnotation)],
    expected: &Expect,
) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;

    let db = &DB::default();
    let example = VdSynExprExample::new(
        input,
        LxMode::Math,
        token_annotations,
        space_annotations,
        db,
    );
    expected.assert_eq(&example.show_display_tree(db));
}

#[test]
fn literal_vd_syn_expr_parsing_works() {
    t(
        "",
        &[],
        &[],
        &expect![[r#"
            "" error
        "#]],
    );
    t(
        "1",
        &[],
        &[],
        &expect![[r#"
            "1" literal
        "#]],
    );
    t(
        "11",
        &[],
        &[],
        &expect![[r#"
            "11" literal
        "#]],
    );
    t(
        "1 1",
        &[],
        &[],
        &expect![[r#"
            "1 1" separated list
            ├─ "1" literal
            └─ "1" literal
        "#]],
    );
}

#[test]
fn basic_arithematics_vd_syn_expr_parsing_works() {
    t(
        "1 + 1",
        &[],
        &[],
        &expect![[r#"
            "1 + 1" separated list
            ├─ "1" literal
            └─ "1" literal
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" separated list
            ├─ "1 + 1" separated list
            │ ├─ "1" literal
            │ └─ "1" literal
            └─ "2" literal
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" separated list
            ├─ "1 + 1" separated list
            │ ├─ "1" literal
            │ └─ "1" literal
            └─ "2" literal
        "#]],
    );
}

#[test]
fn xyz_vd_syn_expr_parsing_works() {
    t(
        "x",
        &[],
        &[],
        &expect![[r#"
            "x" letter
        "#]],
    );
    t(
        "y",
        &[],
        &[],
        &expect![[r#"
            "y" letter
        "#]],
    );
    t(
        "z",
        &[],
        &[],
        &expect![[r#"
            "z" letter
        "#]],
    );
    t(
        "xyz",
        &[],
        &[],
        &expect![[r#"
            "xyz" separated list
            ├─ "x" letter
            ├─ "y" letter
            └─ "z" letter
        "#]],
    );
}

#[test]
fn arithemtics_with_xyz_vd_syn_expr_parsing_works() {
    t(
        "1+x",
        &[],
        &[],
        &expect![[r#"
            "1+x" separated list
            ├─ "1" literal
            └─ "x" letter
        "#]],
    );
    t(
        "1+2x",
        &[],
        &[],
        &expect![[r#"
            "1+2x" separated list
            ├─ "1" literal
            └─ "2x" separated list
              ├─ "2" literal
              └─ "x" letter
        "#]],
    );
    t(
        "1+2xy",
        &[],
        &[],
        &expect![[r#"
            "1+2xy" separated list
            ├─ "1" literal
            └─ "2xy" separated list
              ├─ "2" literal
              ├─ "x" letter
              └─ "y" letter
        "#]],
    );
    t(
        "1+x+2xy",
        &[],
        &[],
        &expect![[r#"
            "1+x+2xy" separated list
            ├─ "1" literal
            ├─ "x" letter
            └─ "2xy" separated list
              ├─ "2" literal
              ├─ "x" letter
              └─ "y" letter
        "#]],
    );
    t(
        "1+x+2",
        &[],
        &[],
        &expect![[r#"
            "1+x+2" separated list
            ├─ "1" literal
            ├─ "x" letter
            └─ "2" literal
        "#]],
    );
    t(
        "1+x+2y",
        &[],
        &[],
        &expect![[r#"
            "1+x+2y" separated list
            ├─ "1" literal
            ├─ "x" letter
            └─ "2y" separated list
              ├─ "2" literal
              └─ "y" letter
        "#]],
    );
    t(
        "1+x+y+z+xyz",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+xyz" separated list
            ├─ "1" literal
            ├─ "x" letter
            ├─ "y" letter
            ├─ "z" letter
            └─ "xyz" separated list
              ├─ "x" letter
              ├─ "y" letter
              └─ "z" letter
        "#]],
    );
    t(
        "1+x+y+z+abc",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+abc" separated list
            ├─ "1" literal
            ├─ "x" letter
            ├─ "y" letter
            ├─ "z" letter
            └─ "abc" separated list
              ├─ "a" letter
              ├─ "b" letter
              └─ "c" letter
        "#]],
    );
}

#[test]
fn delimiters_vd_syn_expr_parsing_works() {
    t(
        "(1)",
        &[],
        &[],
        &expect![[r#"
            "(1)" delimited
            └─ "1" literal
        "#]],
    );
}

#[test]
fn math_zero_argument_commands_vd_syn_expr_parsing_works() {
    t(
        "\\alpha",
        &[],
        &[],
        &expect![[r#"
            "\\alpha" letter
        "#]],
    );
    t(
        "\\pi",
        &[],
        &[],
        &expect![[r#"
            "\\pi" letter
        "#]],
    );
}

#[test]
fn math_commands_with_arguments_vd_syn_expr_parsing_works() {
    t(
        "\\frac{1}{2}",
        &[],
        &[],
        &expect![[r#"
            "\\frac{1}{2}" fraction
            ├─ "1" literal
            └─ "2" literal
        "#]],
    );
    t(
        "\\frac{x+1}{2-2y}",
        &[],
        &[],
        &expect![[r#"
            "\\frac{x+1}{2-2y}" fraction
            ├─ "x+1" separated list
            │ ├─ "x" letter
            │ └─ "1" literal
            └─ "2-2y" binary
              ├─ "2" literal
              └─ "2y" separated list
                ├─ "2" literal
                └─ "y" letter
        "#]],
    );
    t(
        "\\sqrt{1}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{1}" sqrt
            └─ "1" literal
        "#]],
    );
    t(
        "\\sqrt{x+1}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{x+1}" sqrt
            └─ "x+1" separated list
              ├─ "x" letter
              └─ "1" literal
        "#]],
    );
    t(
        "\\sqrt{\\frac{1}{2+x}}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{\\frac{1}{2+x}}" sqrt
            └─ "\\frac{1}{2+x}" fraction
              ├─ "1" literal
              └─ "2+x" separated list
                ├─ "2" literal
                └─ "x" letter
        "#]],
    );
}

#[test]
fn debug_vd_syn_expr_parsing_works() {
    t(
        "1+x+2y",
        &[],
        &[],
        &expect![[r#"
            "1+x+2y" separated list
            ├─ "1" literal
            ├─ "x" letter
            └─ "2y" separated list
              ├─ "2" literal
              └─ "y" letter
        "#]],
    );
}

#[test]
fn opr_commands_vd_syn_expr_parsing_works() {
    use visored_annotation::annotation::token::DIFFERENTIAL;

    t(
        "\\int xdx",
        &[(("\\int x", "d"), DIFFERENTIAL)],
        &[],
        &expect![[r#"
            "\\int xdx" prefix
            └─ "xdx" separated list
              ├─ "x" letter
              └─ "dx" prefix
                └─ "x" letter
        "#]],
    )
}

#[test]
fn attach_vd_syn_expr_parsing_works() {
    t(
        "x^2",
        &[],
        &[],
        &expect![[r#"
            "x^2" attach
            ├─ "x" letter
            └─ "2" literal
        "#]],
    );
    t(
        "{(x+1)}^2",
        &[],
        &[],
        &expect![[r#"
            "{(x+1)}^2" attach
            ├─ "{(x+1)}" latex delimited
            │ └─ "(x+1)" delimited
            │   └─ "x+1" separated list
            │     ├─ "x" letter
            │     └─ "1" literal
            └─ "2" literal
        "#]],
    );
}
