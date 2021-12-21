use crate::*;
use test_utils::assert_test_env;

#[test]
fn empty_lambda() {
    let mut db = HuskyLangDatabase::new(None);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
|| 1
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id);
    p!(atomized_main_file);
}

#[test]
fn one_argument_lambda() {
    let mut db = HuskyLangDatabase::new(None);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
|x| x
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id);
    p!(atomized_main_file);
}
