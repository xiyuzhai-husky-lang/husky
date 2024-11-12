use super::*;
use crate::test_helpers::example::VdSemExprExample;
use expect_test::{expect, Expect};
use latex_prelude::mode::LxMode;

pub(crate) fn t(input: &str, expected: &Expect) {
    let db = &DB::default();
    let example = VdSemExprExample::new(input, LxMode::Rose, &[], &[], db);
    expected.assert_eq(&example.show_display_tree(db))
}

#[test]
pub(crate) fn basic_vd_sem_expr_works() {
    t(
        "Let $x=1$.",
        &expect![[r#"
            Let $x=1$.
            └─ "Let $x=1$." stmt.paragraph
              └─ "Let $x=1$." sentence.clauses
                └─ "Let $x=1$" clause.let
                  └─ "x=1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Assume $x=1$.",
        &expect![[r#"
            Assume $x=1$.
            └─ "Assume $x=1$." stmt.paragraph
              └─ "Assume $x=1$." sentence.clauses
                └─ "Assume $x=1$" clause.assume
                  └─ "x=1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Then $x=1$.",
        &expect![[r#"
            Then $x=1$.
            └─ "Then $x=1$." stmt.paragraph
              └─ "Then $x=1$." sentence.clauses
                └─ "Then $x=1$" clause.then
                  └─ "x=1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        "Let $x=1$. Assume $y=2$. Then $x+y=3$.",
        &expect![[r#"
            Let $x=1$. Assume $y=2$. Then $x+y=3$.
            └─ "Let $x=1$. Assume $y=2$. Then $x+y=3$." stmt.paragraph
              ├─ "Let $x=1$." sentence.clauses
              │ └─ "Let $x=1$" clause.let
              │   └─ "x=1" expr.separated_list
              │     ├─ "x" expr.letter
              │     └─ "1" expr.literal
              ├─ "Assume $y=2$." sentence.clauses
              │ └─ "Assume $y=2$" clause.assume
              │   └─ "y=2" expr.separated_list
              │     ├─ "y" expr.letter
              │     └─ "2" expr.literal
              └─ "Then $x+y=3$." sentence.clauses
                └─ "Then $x+y=3$" clause.then
                  └─ "x+y=3" expr.separated_list
                    ├─ "x+y" expr.separated_list
                    │ ├─ "x" expr.letter
                    │ └─ "y" expr.letter
                    └─ "3" expr.literal
        "#]],
    );
}
