#[cfg(test)]
use crate::*;

#[test]
fn haha() {
    let mut db = Comptime::new_default(__resolve_root_defn);
    let source: &'static str = r#"
task:
    ml::datasets::synthetic::trivial::real1d::dataset1()

main:
    if input > 0.0:
        1
    else:
        0
"#;

    db.set_live_file_text("haha/main.hsy".into(), source.into());
    let main_file_id = db.intern_file("haha/main.hsy".into());
    let ast_text = db.ast_text(main_file_id).unwrap();
    test_print!(ast_text.errors());
}
