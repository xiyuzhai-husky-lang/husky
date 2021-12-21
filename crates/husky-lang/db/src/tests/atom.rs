use crate::*;

use test_utils::assert_test_env;

#[test]
fn no_error_single_file() {
    let mut db = HuskyLangDatabase::new(None);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id);
    p!(atomized_main_file);
}
mod builtin;
mod lambda;
mod utils;
