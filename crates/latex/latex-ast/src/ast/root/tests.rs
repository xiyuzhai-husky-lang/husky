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
        "\\documentclass{article}",
        expect![[r#"
            "\\documentclass{article}" all input
            └─ "\\documentclass{article}" complete command
              └─ article
        "#]],
    );
    t(
        "\\usepackage{amsmath}",
        expect![[r#"
            "\\usepackage{amsmath}" all input
            └─ "\\usepackage{amsmath}" complete command
              └─ amsmath
        "#]],
    );
    t(
        "\\usepackage[utf8]{inputenc}",
        expect![[r#"
            "\\usepackage[utf8]{inputenc}" all input
            └─ "\\usepackage[utf8]{inputenc}" complete command
              └─ inputenc
        "#]],
    );
}

#[test]
fn parse_document_environment_works() {
    t(
        r#"\begin{document}\end{document}"#,
        expect![[r#"
        "\\begin{document}\\end{document}" all input
        └─ "\\begin{document}\\end{document}" environment
    "#]],
    );
    t(
        r#"\begin{document}Hello\end{document}"#,
        expect![[r#"
            "\\begin{document}Hello\\end{document}" all input
            └─ "\\begin{document}Hello\\end{document}" environment
              └─ "Hello" word
        "#]],
    );
    t(
        r#"\begin{document}Let $x\in\mathbb{R}$.\end{document}"#,
        expect![[r#"
            "\\begin{document}Let $x\\in\\mathbb{R}$.\\end{document}" all input
            └─ "\\begin{document}Let $x\\in\\mathbb{R}$.\\end{document}" environment
              ├─ "Let" word
              ├─ "$x\\in\\mathbb{R}$" math
              │ ├─ "x" plain letter
              │ ├─ "\\in" complete command
              │ └─ "\\mathbb{R}" styled letter
              └─ "." punctuation
        "#]],
    );
}
