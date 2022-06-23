use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

context:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );

    let main_file_id = db.intern_file("haha/main.hsk".into());
    let _pack = db.package(main_file_id).unwrap();
    let _main_block = db.main_feature_repr(main_file_id).unwrap();
}
