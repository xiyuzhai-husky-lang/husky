use super::*;
use expect_test::{expect, Expect};
use helpers::tracker::VdMirExprTracker;
use latex_prelude::helper::tracker::LxDocumentBodyInput;
use latex_prelude::mode::LxMode;
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(content: &str, expect: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &DB::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(db, PathBuf::from(file!()));
    let tracker = VdMirExprTracker::new(
        LxDocumentBodyInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        db,
    );
    expect.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn basic_body_to_vd_mir_works() {
    t(
        r#"Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(Id { value: 2 }))
              └─ block: Paragraph
                └─ block: Sentence
                  └─ let placeholder
        "#]],
    );
    t(
        r#"\begin{example}\end{example}"#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(Id { value: 2 }))
              └─ block: Environment(LxEnvironmentPath { name: LxEnvironmentName(Coword(Id { value: 46 })) }, VdModulePath(Id { value: 3 }))
        "#]],
    );
    t(
        r#"\begin{example}Let $x\in\mathbb{R}$.\end{example}"#,
        &expect![[r#"
            └─ block: Division(Stmts, VdModulePath(Id { value: 2 }))
              └─ block: Environment(LxEnvironmentPath { name: LxEnvironmentName(Coword(Id { value: 46 })) }, VdModulePath(Id { value: 3 }))
                └─ block: Paragraph
                  └─ block: Sentence
                    └─ let placeholder
        "#]],
    );
}
