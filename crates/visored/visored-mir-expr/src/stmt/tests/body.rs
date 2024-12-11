use super::*;
use eterned::db::EternerDb;
use expect_test::{expect, Expect};
use helpers::tracker::VdMirExprTracker;
use latex_prelude::helper::tracker::LxDocumentBodyInput;
use latex_prelude::mode::LxMode;
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(llm: &VdLlm, content: &str, expect: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let tracker = VdMirExprTracker::new(
        LxDocumentBodyInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        llm,
        db,
    );
    expect.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn basic_body_to_vd_mir_works() {
    let llm = &VdLlm::new();
    t(
        llm,
        r#"Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(`root.stmts1`))
              └─ block: Paragraph
                └─ block: Sentence
                  └─ let placeholder
        "#]],
    );
    t(
        llm,
        r#"\begin{example}\end{example}"#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(`root.stmts1`))
              └─ block: Environment(LxEnvironmentPath { name: LxEnvironmentName(Coword("example")) }, Example, VdModulePath(`root.stmts1.example1`))
        "#]],
    );
    t(
        llm,
        r#"\begin{example}Let $x\in\mathbb{R}$.\end{example}"#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(`root.stmts1`))
              └─ block: Environment(LxEnvironmentPath { name: LxEnvironmentName(Coword("example")) }, Example, VdModulePath(`root.stmts1.example1`))
                └─ block: Paragraph
                  └─ block: Sentence
                    └─ let placeholder
        "#]],
    );
}
