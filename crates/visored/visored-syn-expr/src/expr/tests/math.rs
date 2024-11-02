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
fn litera_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "",
        &[],
        &[],
        &expect![[r#"
            ""
        "#]],
    );
    t(
        "1",
        &[],
        &[],
        &expect![[r#"
            "1"
        "#]],
    );
    t(
        "11",
        &[],
        &[],
        &expect![[r#"
            "11"
        "#]],
    );
    t(
        "1 1",
        &[],
        &[],
        &expect![[r#"
                "1 1"
                ├─ "1"
                └─ "1"
            "#]],
    );
}

#[test]
fn basic_arithematics_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "1 + 1",
        &[],
        &[],
        &expect![[r#"
            "1 + 1"
            ├─ "1"
            └─ "1"
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2"
            ├─ "1 + 1"
            │ ├─ "1"
            │ └─ "1"
            └─ "2"
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2"
            ├─ "1 + 1"
            │ ├─ "1"
            │ └─ "1"
            └─ "2"
        "#]],
    );
}

#[test]
fn xyz_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "x",
        &[],
        &[],
        &expect![[r#"
            "x"
        "#]],
    );
    t(
        "y",
        &[],
        &[],
        &expect![[r#"
            "y"
        "#]],
    );
    t(
        "z",
        &[],
        &[],
        &expect![[r#"
            "z"
        "#]],
    );
    t(
        "xyz",
        &[],
        &[],
        &expect![[r#"
            "xyz"
            ├─ "xy"
            │ ├─ "x"
            │ └─ "y"
            └─ "z"
        "#]],
    );
}

#[test]
fn arithemtics_with_xyz_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "1+x",
        &[],
        &[],
        &expect![[r#"
            "1+x"
            ├─ "1"
            └─ "x"
        "#]],
    );
    t(
        "1+2x",
        &[],
        &[],
        &expect![[r#"
            "1+2x"
            ├─ "1"
            └─ "2x"
              ├─ "2"
              └─ "x"
        "#]],
    );
    t(
        "1+x+y+z+xyz",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+xyz"
            ├─ "1+x+y+z"
            │ ├─ "1+x+y"
            │ │ ├─ "1+x"
            │ │ │ ├─ "1"
            │ │ │ └─ "x"
            │ │ └─ "y"
            │ └─ "z"
            └─ "xyz"
              ├─ "xy"
              │ ├─ "x"
              │ └─ "y"
              └─ "z"
        "#]],
    );
}
