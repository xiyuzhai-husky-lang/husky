use common::*;
use husky_lang_db::*;

use crate::session::Session;

#[test]
fn simple() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );
    let main_file = db.intern_file("haha/main.hsk".into());
    let ast_text = db.ast_text(main_file);
    let package = db.package(main_file).unwrap();
    let sess = Session::new(&package);
}
