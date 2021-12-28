use crate::*;

#[test]
fn func() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
func f(x: i32, y: i32) -> i32:
    return x + y
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let fmt_text = db.fmt_text(main_file_id).unwrap();
    ep!(fmt_text);
}
