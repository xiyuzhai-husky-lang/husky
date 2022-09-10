mod func;
mod lambda;
mod utils;
#[cfg(test)]
use crate::*;
#[cfg(test)]
#[test]
fn no_error_single_file() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    db.set_live_file_text(
        "haha/main.hsy".into(),
        r#"
struct A:
    a: i32

main:
    a = 1
    b = (1 * a - 2) * -1
    assert a == b
"#
        .into(),
    );

    let main_file_id = db.intern_file("haha/main.hsy".into());
    let fmt_text = db.fmt_text(main_file_id).unwrap();
    ep!(fmt_text);
}
