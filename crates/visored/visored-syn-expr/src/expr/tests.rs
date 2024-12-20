use super::*;
use crate::helpers::tracker::VdSynExprTracker;
use eterned::db::EternerDb;
use expect_test::{expect, Expect};
use latex_prelude::{helper::tracker::LxFormulaInput, mode::LxMode};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

fn t(
    models: &VdModels,
    content: &str,
    token_annotations: &[((&str, &str), VdTokenAnnotation)],
    space_annotations: &[((&str, &str), VdSpaceAnnotation)],
    expected: &Expect,
) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let vibe = VdSynExprVibe::ROOT_CNL;
    let tracker = VdSynExprTracker::new(
        LxFormulaInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        token_annotations,
        space_annotations,
        models,
        vibe,
        db,
    );
    expected.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn literal_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "",
        &[],
        &[],
        &expect![[r#"
            "" expr.error
        "#]],
    );
    t(
        models,
        "1",
        &[],
        &[],
        &expect![[r#"
            "1" expr.literal
        "#]],
    );
    t(
        models,
        "11",
        &[],
        &[],
        &expect![[r#"
            "11" expr.literal
        "#]],
    );
    t(
        models,
        "1 1",
        &[],
        &[],
        &expect![[r#"
            "1 1" expr.separated_list
            ├─ "1" expr.literal
            └─ "1" expr.literal
        "#]],
    );
}

#[test]
fn basic_arithematics_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "1 + 1",
        &[],
        &[],
        &expect![[r#"
            "1 + 1" expr.separated_list
            ├─ "1" expr.literal
            └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" expr.separated_list
            ├─ "1 + 1" expr.separated_list
            │ ├─ "1" expr.literal
            │ └─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
    t(
        models,
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" expr.separated_list
            ├─ "1 + 1" expr.separated_list
            │ ├─ "1" expr.literal
            │ └─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
    t(
        models,
        "+1",
        &[],
        &[],
        &expect![[r#"
            "+1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "-1",
        &[],
        &[],
        &expect![[r#"
            "-1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
}

#[test]
fn relationship_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "1<2",
        &[],
        &[],
        &expect![[r#"
            "1<2" expr.separated_list
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn xyz_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "x",
        &[],
        &[],
        &expect![[r#"
            "x" expr.letter
        "#]],
    );
    t(
        models,
        "y",
        &[],
        &[],
        &expect![[r#"
            "y" expr.letter
        "#]],
    );
    t(
        models,
        "z",
        &[],
        &[],
        &expect![[r#"
            "z" expr.letter
        "#]],
    );
    t(
        models,
        "xyz",
        &[],
        &[],
        &expect![[r#"
            "xyz" expr.separated_list
            ├─ "x" expr.letter
            ├─ "y" expr.letter
            └─ "z" expr.letter
        "#]],
    );
}

#[test]
fn arithemtics_with_xyz_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "1+x",
        &[],
        &[],
        &expect![[r#"
            "1+x" expr.separated_list
            ├─ "1" expr.literal
            └─ "x" expr.letter
        "#]],
    );
    t(
        models,
        "1+2x",
        &[],
        &[],
        &expect![[r#"
            "1+2x" expr.separated_list
            ├─ "1" expr.literal
            └─ "2x" expr.separated_list
              ├─ "2" expr.literal
              └─ "x" expr.letter
        "#]],
    );
    t(
        models,
        "1+2xy",
        &[],
        &[],
        &expect![[r#"
            "1+2xy" expr.separated_list
            ├─ "1" expr.literal
            └─ "2xy" expr.separated_list
              ├─ "2" expr.literal
              ├─ "x" expr.letter
              └─ "y" expr.letter
        "#]],
    );
    t(
        models,
        "1+x+2xy",
        &[],
        &[],
        &expect![[r#"
            "1+x+2xy" expr.separated_list
            ├─ "1" expr.literal
            ├─ "x" expr.letter
            └─ "2xy" expr.separated_list
              ├─ "2" expr.literal
              ├─ "x" expr.letter
              └─ "y" expr.letter
        "#]],
    );
    t(
        models,
        "1+x+2",
        &[],
        &[],
        &expect![[r#"
            "1+x+2" expr.separated_list
            ├─ "1" expr.literal
            ├─ "x" expr.letter
            └─ "2" expr.literal
        "#]],
    );
    t(
        models,
        "1+x+2y",
        &[],
        &[],
        &expect![[r#"
            "1+x+2y" expr.separated_list
            ├─ "1" expr.literal
            ├─ "x" expr.letter
            └─ "2y" expr.separated_list
              ├─ "2" expr.literal
              └─ "y" expr.letter
        "#]],
    );
    t(
        models,
        "1+x+y+z+xyz",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+xyz" expr.separated_list
            ├─ "1" expr.literal
            ├─ "x" expr.letter
            ├─ "y" expr.letter
            ├─ "z" expr.letter
            └─ "xyz" expr.separated_list
              ├─ "x" expr.letter
              ├─ "y" expr.letter
              └─ "z" expr.letter
        "#]],
    );
    t(
        models,
        "1+x+y+z+abc",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+abc" expr.separated_list
            ├─ "1" expr.literal
            ├─ "x" expr.letter
            ├─ "y" expr.letter
            ├─ "z" expr.letter
            └─ "abc" expr.separated_list
              ├─ "a" expr.letter
              ├─ "b" expr.letter
              └─ "c" expr.letter
        "#]],
    );
}

#[test]
fn more_operators_with_xyz_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "0\\ne1",
        &[],
        &[],
        &expect![[r#"
            "0\\ne1" expr.separated_list
            ├─ "0" expr.literal
            └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "0\\in\\mathbb{N}",
        &[],
        &[],
        &expect![[r#"
            "0\\in\\mathbb{N}" expr.separated_list
            ├─ "0" expr.literal
            └─ "\\mathbb{N}" expr.letter
        "#]],
    );
    t(
        models,
        "A\\subset B",
        &[],
        &[],
        &expect![[r#"
            "A\\subset B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\subseteq B",
        &[],
        &[],
        &expect![[r#"
            "A\\subseteq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\subseteqq B",
        &[],
        &[],
        &expect![[r#"
            "A\\subseteqq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\supseteq B",
        &[],
        &[],
        &expect![[r#"
            "A\\supseteq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\supseteqq B",
        &[],
        &[],
        &expect![[r#"
            "A\\supseteqq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\subsetneq B",
        &[],
        &[],
        &expect![[r#"
            "A\\subsetneq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
    t(
        models,
        "A\\supsetneq B",
        &[],
        &[],
        &expect![[r#"
            "A\\supsetneq B" expr.separated_list
            ├─ "A" expr.letter
            └─ "B" expr.letter
        "#]],
    );
}

#[test]
fn delimiters_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "(1)",
        &[],
        &[],
        &expect![[r#"
            "(1)" delimited
            └─ "1" expr.literal
        "#]],
    );
}

#[test]
fn math_zero_argument_commands_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "\\alpha",
        &[],
        &[],
        &expect![[r#"
            "\\alpha" expr.letter
        "#]],
    );
    t(
        models,
        "\\pi",
        &[],
        &[],
        &expect![[r#"
            "\\pi" expr.letter
        "#]],
    );
}

#[test]
fn math_commands_with_arguments_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "\\frac{1}{2}",
        &[],
        &[],
        &expect![[r#"
            "\\frac{1}{2}" fraction
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
    t(
        models,
        "\\frac{x+1}{2-2y}",
        &[],
        &[],
        &expect![[r#"
            "\\frac{x+1}{2-2y}" fraction
            ├─ "x+1" expr.separated_list
            │ ├─ "x" expr.letter
            │ └─ "1" expr.literal
            └─ "2-2y" expr.binary
              ├─ "2" expr.literal
              └─ "2y" expr.separated_list
                ├─ "2" expr.literal
                └─ "y" expr.letter
        "#]],
    );
    t(
        models,
        "\\sqrt{1}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{1}" sqrt
            └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "\\sqrt{x+1}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{x+1}" sqrt
            └─ "x+1" expr.separated_list
              ├─ "x" expr.letter
              └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "\\sqrt{\\frac{1}{2+x}}",
        &[],
        &[],
        &expect![[r#"
            "\\sqrt{\\frac{1}{2+x}}" sqrt
            └─ "\\frac{1}{2+x}" fraction
              ├─ "1" expr.literal
              └─ "2+x" expr.separated_list
                ├─ "2" expr.literal
                └─ "x" expr.letter
        "#]],
    );
}

#[test]
fn opr_commands_vd_syn_expr_parsing_works() {
    use visored_annotation::annotation::token::DIFFERENTIAL;

    let models = &VdModels::new();
    t(
        models,
        "\\int xdx",
        &[(("\\int x", "d"), DIFFERENTIAL)],
        &[],
        &expect![[r#"
            "\\int xdx" expr.prefix
            └─ "xdx" expr.separated_list
              ├─ "x" expr.letter
              └─ "dx" expr.prefix
                └─ "x" expr.letter
        "#]],
    )
}

#[test]
fn attach_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "x^2",
        &[],
        &[],
        &expect![[r#"
            "x^2" expr.attach
            ├─ "x" expr.letter
            └─ "2" expr.literal
        "#]],
    );
    t(
        models,
        "{(x+1)}^2",
        &[],
        &[],
        &expect![[r#"
            "{(x+1)}^2" expr.attach
            ├─ "{(x+1)}" expr.latex_delimited
            │ └─ "(x+1)" delimited
            │   └─ "x+1" expr.separated_list
            │     ├─ "x" expr.letter
            │     └─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn styled_letter_vd_syn_expr_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "\\mathbb{N}",
        &[],
        &[],
        &expect![[r#"
            "\\mathbb{N}" expr.letter
        "#]],
    );
}
