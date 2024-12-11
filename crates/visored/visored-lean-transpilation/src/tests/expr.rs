use super::*;
use crate::{
    helpers::tracker::VdLeanTranspilationTracker, scheme::sparse::VdLeanTranspilationSparseScheme,
};
use eterned::db::EternerDb;
use latex_prelude::{helper::tracker::LxFormulaInput, mode::LxMode};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(llm: &VdLlm, content: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let tracker = VdLeanTranspilationTracker::new(
        LxFormulaInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        llm,
        db,
        &VdLeanTranspilationSparseScheme,
    );
    expected_display_tree.assert_eq(&tracker.show_display_tree(db));
    expected_fmt.assert_eq(&tracker.show_fmt(db));
}

#[test]
fn basic_visored_expr_to_lean_works() {
    let llm = &VdLlm::new();
    t(
        llm,
        "1",
        &expect![[r#"
            literal: `1`
        "#]],
        &expect!["1"],
    );
    t(
        llm,
        "-1",
        &expect![[r#"
            application
            └─ literal: `1`
        "#]],
        &expect!["-1"],
    );
    t(
        llm,
        "1 + 1",
        &expect![[r#"
            application
            ├─ literal: `1`
            └─ literal: `1`
        "#]],
        &expect!["1 + 1"],
    );
    t(
        llm,
        "1 < 2",
        &expect![[r#"
            application
            ├─ literal: `1`
            └─ literal: `2`
        "#]],
        &expect!["1 < 2"],
    );
    t(
        llm,
        "1\\in\\mathbb{N}",
        &expect![[r#"
            application
            ├─ literal: `1`
            └─ item path: `ℕ`
        "#]],
        &expect!["sorry"],
    );
    t(
        llm,
        "\\frac{1}{2}",
        &expect![[r#"
            application
            ├─ literal: `1`
            └─ literal: `2`
        "#]],
        &expect!["1 / 2"],
    );
    t(
        llm,
        "\\sqrt{2}",
        &expect![[r#"
            application
            ├─ item path: `√`
            └─ literal: `2`
        "#]],
        &expect!["√ 2"],
    );
}

#[test]
fn item_path_to_lean_works() {
    let llm = &VdLlm::new();
    t(
        llm,
        "\\mathbb{N}",
        &expect![[r#"
            item path: `ℕ`
        "#]],
        &expect!["ℕ"],
    );
}
