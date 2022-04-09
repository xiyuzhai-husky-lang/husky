use compile_time_db::*;

fn walkthrough(main_source: &'static str) {
    let mut db = HuskyLangCompileTime::default();
    db.set_live_file_text("haha/main.hsk".into(), main_source.into());
    let main_file = db.intern_file("haha/main.hsk".into());
    let pack = db.package(main_file).unwrap();
    // let sess = Session::new(&Pack);
}

#[test]
fn use_assert() {
    walkthrough(
        r#"
dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#,
    )
}

#[test]
fn use_input() {
    walkthrough(
        r#"
dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    c = input
    a
"#,
    )
}
