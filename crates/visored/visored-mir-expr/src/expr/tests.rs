use crate::elaborator::linear::VdMirSequentialElaborator;
use crate::*;
use elaborator::VdMirTrivialElaborator;
use eterned::db::EternerDb;
use expect_test::{expect, Expect};
use helpers::tracker::VdMirExprTracker;
use latex_prelude::helper::tracker::LxFormulaInput;
use latex_prelude::mode::LxMode;
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_syn_expr::vibe::VdSynExprVibe;

fn t(content: &str, expect: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let tracker = VdMirExprTracker::new(
        LxFormulaInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        &VdModels::new(),
        VdSynExprVibe::ROOT_CNL,
        db,
        |_| VdMirTrivialElaborator::default(),
    );
    expect.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn basic_to_vd_mir_works() {
    t(
        "1 + 1",
        &expect![[r#"
            folding separated list
            ├─ 1
            └─ 1
        "#]],
    );
    t(
        "1",
        &expect![[r#"
            1
        "#]],
    );
    t(
        "-1",
        &expect![[r#"
            prefix opr
            └─ 1
        "#]],
    );
    t(
        "1<2",
        &expect![[r#"
            folding separated list
            ├─ 1
            └─ 2
        "#]],
    );
}

#[test]
fn frac_to_vd_mir_works() {
    t(
        "\\frac{1}{2}",
        &expect![[r#"
            binary opr
            ├─ 1
            └─ 2
        "#]],
    );
}

#[test]
fn sqrt_to_vd_mir_works() {
    t(
        "\\sqrt{1}",
        &expect![[r#"
        sqrt
        └─ 1
    "#]],
    );
}
