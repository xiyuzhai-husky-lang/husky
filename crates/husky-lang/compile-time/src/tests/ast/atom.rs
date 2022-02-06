mod builtin;
mod generics;
mod lambda;
mod utils;

use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyLangDatabase::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

main:
    let a = 1
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let atomized_main_file = db.atomized_text(main_file_id);
    p!(atomized_main_file);
}
