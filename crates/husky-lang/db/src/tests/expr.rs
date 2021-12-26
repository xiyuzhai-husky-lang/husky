use crate::*;

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
    todo!();
    // let expr_text = db.expr_text(main_file_id);
    // ep!(expr_text);
}
