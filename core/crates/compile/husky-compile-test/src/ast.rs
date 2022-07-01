mod atom;

use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyCompileTime::new(__root_defn);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );

    let main_file_id = db.intern_file("haha/main.hsk".into());
    let ast_text = db.ast_text(main_file_id).unwrap();
    should_eq!(ast_text.errors().len(), 0);
}
