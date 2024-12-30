use super::*;
use crate::{
    helpers::tracker::VdLeanTranspilationTracker, scheme::dense::VdLeanTranspilationDenseScheme,
};
use eterned::db::EternerDb;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxPageInput},
    mode::LxMode,
};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_mir_expr::tactic::elaboration::VdMirTacticTrivialElaborator;
use visored_models::VdModels;
use visored_syn_expr::vibe::VdSynExprVibe;

fn t(models: &VdModels, content: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let tracker = VdLeanTranspilationTracker::new(
        LxDocumentBodyInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        models,
        VdSynExprVibe::ROOT_CNL,
        db,
        &VdLeanTranspilationDenseScheme,
        VdMirTacticTrivialElaborator::default(),
    );
    expected_display_tree.assert_eq(&tracker.show_display_tree(db));
    expected_fmt.assert_eq(&tracker.show_fmt(db));
}

#[test]
fn basic_body_to_lean_works() {
    let models = &VdModels::new();
    t(
        models,
        "Let $x\\in\\mathbb{N}$.",
        &expect![[r#"
            └─ group: `division`
              └─ def: `h`
                ├─ item path: `ℕ`
                └─ tactics
                  └─ tactic: `Obvious`
        "#]],
        &expect![[r#"
            def h(x : ℕ) := by
              obvious"#]],
    );
    t(
        models,
        r#"\begin{example}\end{example}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `environment`
                └─ def: `h`
                  └─ tactics
                    └─ tactic: `Obvious`
        "#]],
        &expect![[r#"
            namespace Example1
            def h := by
              obvious
            end Example1
        "#]],
    );
    t(
        models,
        r#"\begin{example}Let $x\in\mathbb{R}$.\end{example}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `environment`
                └─ def: `h`
                  ├─ item path: `ℝ`
                  └─ tactics
                    └─ tactic: `Obvious`
        "#]],
        &expect![[r#"
            namespace Example1
            def h(x : ℝ) := by
              obvious
            end Example1
        "#]],
    );
    t(
        models,
        r#"\section{Introduction}Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `division`
                └─ def: `h`
                  ├─ item path: `ℝ`
                  └─ tactics
                    └─ tactic: `Obvious`
        "#]],
        &expect![[r#"
            namespace Section1
            def h(x : ℝ) := by
              obvious
            end Section1
        "#]],
    );
    t(
        models,
        r#"\section{Introduction}Let $x\in\mathbb{R}$.\subsection{Hello}Let $y\in\mathbb{R}$.\subsection{World}\subsection{This}\subsubsection{Is}\subsubsection{Bad}"#,
        &expect![[r#"
            └─ group: `division`
              ├─ group: `division`
              │ └─ def: `h`
              │   ├─ item path: `ℝ`
              │   └─ tactics
              │     └─ tactic: `Obvious`
              ├─ group: `division`
              │ └─ group: `division`
              │   └─ def: `h`
              │     ├─ item path: `ℝ`
              │     └─ tactics
              │       └─ tactic: `Obvious`
              ├─ group: `division`
              └─ group: `division`
                ├─ group: `division`
                └─ group: `division`
        "#]],
        &expect![[r#"
            namespace Section1
            def h(x : ℝ) := by
              obvious

            namespace Subsection1
            def h(y : ℝ) := by
              obvious
            end Subsection1

            namespace Subsection2
            end Subsection2

            namespace Subsection3
            namespace Subsubsection1
            end Subsubsection1

            namespace Subsubsection2
            end Subsubsection2
            end Subsection3
            end Section1
        "#]],
    );
}
