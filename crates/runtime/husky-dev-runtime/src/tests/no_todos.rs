use husky_dev_comptime::*;

fn walkthrough(main_source: &'static str) {
    let mut db = husky - compilerompileTime::new_default(__root_defn);
    db.set_live_file_text("haha/main.hsy".into(), main_source.into());
    let target_entrance = db.intern_path("haha/main.hsy".into());
    let pack = db.package(target_entrance).unwrap();
    // let sess = Session::new(&Pack);
}

#[test]
fn use_assert() {
    walkthrough(
        r#"
context:
    ml::datasets::synthetic::trivial::real1d::dataset1()

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
context:
    ml::datasets::synthetic::trivial::real1d::dataset1()

main:
    a = 1
    c = input
    a
"#,
    )
}
