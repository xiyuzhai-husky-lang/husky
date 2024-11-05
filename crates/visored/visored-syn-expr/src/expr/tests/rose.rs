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
        LxMode::Rose,
        token_annotations,
        space_annotations,
        db,
    );
    expected.assert_eq(&example.show_display_tree(db));
}

#[test]
fn let_sentence_vd_syn_expr_parsing_works() {
    t(
        "Let $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Let $x = 1$.
            └─ "Let $x = 1$." stmt.paragraph
              └─ "Let $x = 1$." sentence.clauses
                └─ "Let $x = 1$" clause.let
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Assume $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Assume $x = 1$.
            └─ "Assume $x = 1$." stmt.paragraph
              └─ "Assume $x = 1$." sentence.clauses
                └─ "Assume $x = 1$" clause.assume
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Suppose $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Suppose $x = 1$.
            └─ "Suppose $x = 1$." stmt.paragraph
              └─ "Suppose $x = 1$." sentence.clauses
                └─ "Suppose $x = 1$" clause.assume
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Then $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Then $x = 1$.
            └─ "Then $x = 1$." stmt.paragraph
              └─ "Then $x = 1$." sentence.clauses
                └─ "Then $x = 1$" clause.then
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Let $x = 1$. Assume $x = 1$. Then $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Let $x = 1$. Assume $x = 1$. Then $x = 1$.
            └─ "Let $x = 1$. Assume $x = 1$. Then $x = 1$." stmt.paragraph
              ├─ "Let $x = 1$." sentence.clauses
              │ └─ "Let $x = 1$" clause.let
              │   └─ "x = 1" expr.separated_list
              │     ├─ "x" expr.letter
              │     └─ "1" expr.literal
              ├─ "Assume $x = 1$." sentence.clauses
              │ └─ "Assume $x = 1$" clause.assume
              │   └─ "x = 1" expr.separated_list
              │     ├─ "x" expr.letter
              │     └─ "1" expr.literal
              └─ "Then $x = 1$." sentence.clauses
                └─ "Then $x = 1$" clause.then
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
}
