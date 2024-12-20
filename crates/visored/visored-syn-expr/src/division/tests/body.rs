use super::*;
use eterned::db::EternerDb;
use latex_prelude::helper::tracker::LxDocumentBodyInput;

fn t(models: &VdModels, content: &str, expected: &Expect) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let vibe = VdSynExprVibe::ROOT_CNL;
    let tracker = VdSynExprTracker::new(
        LxDocumentBodyInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        models,
        vibe,
        db,
    );
    expected.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn parse_vd_syn_divisions_works() {
    let models = &VdModels::new();
    t(
        models,
        r#"Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ "Let $x\\in\\mathbb{R}$." division.stmts
              └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                  └─ "Let $x\\in\\mathbb{R}$" clause.let
                    └─ "x\\in\\mathbb{R}" expr.separated_list
                      ├─ "x" expr.letter
                      └─ "\\mathbb{R}" expr.letter
        "#]],
    );
    t(
        models,
        r#"\section{Introduction}Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ "\\section{Introduction}Let $x\\in\\mathbb{R}$." division.section
              └─ "Let $x\\in\\mathbb{R}$." division.stmts
                └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                  └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                    └─ "Let $x\\in\\mathbb{R}$" clause.let
                      └─ "x\\in\\mathbb{R}" expr.separated_list
                        ├─ "x" expr.letter
                        └─ "\\mathbb{R}" expr.letter
        "#]],
    );
    t(
        models,
        r#"\section{Introduction}Let $x\in\mathbb{R}$.\subsection{Hello}Let $y\in\mathbb{R}$.\subsection{World}\subsection{This}\subsubsection{Is}\subsubsection{Bad}"#,
        &expect![[r#"
            └─ "\\section{Introduction}Let $x\\in\\mathbb{R}$.\\subsection{Hello}Let $y\\in\\mathbb{R}$.\\subsection{World}\\subsection{This}\\subsubsection{Is}\\subsubsection{Bad}" division.section
              ├─ "Let $x\\in\\mathbb{R}$." division.stmts
              │ └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
              │   └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
              │     └─ "Let $x\\in\\mathbb{R}$" clause.let
              │       └─ "x\\in\\mathbb{R}" expr.separated_list
              │         ├─ "x" expr.letter
              │         └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{Hello}Let $y\\in\\mathbb{R}$." division.subsection
              │ └─ "Let $y\\in\\mathbb{R}$." division.stmts
              │   └─ "Let $y\\in\\mathbb{R}$." stmt.paragraph
              │     └─ "Let $y\\in\\mathbb{R}$." sentence.clauses
              │       └─ "Let $y\\in\\mathbb{R}$" clause.let
              │         └─ "y\\in\\mathbb{R}" expr.separated_list
              │           ├─ "y" expr.letter
              │           └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{World}" division.subsection
              └─ "\\subsection{This}\\subsubsection{Is}\\subsubsection{Bad}" division.subsection
                ├─ "\\subsubsection{Is}" division.subsubsection
                └─ "\\subsubsection{Bad}" division.subsubsection
        "#]],
    );
}
