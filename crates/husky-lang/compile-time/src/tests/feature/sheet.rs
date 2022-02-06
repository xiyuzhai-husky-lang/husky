use feature::{eval_feature_block, FeatureSheet};
use vm::Conditional;

use crate::*;

#[test]
fn eval_sheet() {
    let mut db = HuskyLangCompileTime::default();
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

    let main_file = db.alloc_file("haha/main.hsk".into());
    let main_feature_block = db.main_feature_block(main_file).unwrap();
    let mut sheet = FeatureSheet::default();
    let input = Ok(Conditional::Undefined);
    let result = eval_feature_block(input, &mut sheet, &main_feature_block)
        .unwrap()
        .defined()
        .unwrap()
        .as_primitive()
        .unwrap();
    should_eq!(result, 1.into());
}
