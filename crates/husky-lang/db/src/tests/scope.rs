use crate::*;

use test_utils::assert_test_env;

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
    let package_scope_id = db.scope_to_id(Scope::new(
        db.string_to_word("haha".into()).ident().unwrap(),
        ScopeParent::Package(main_file_id),
        None,
    ));
    let subscope_table = db.subscope_table(package_scope_id).ok().unwrap();
    assert_eq!(subscope_table.entries.len(), 2);
    assert_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn no_error_many_files() {
    let mut db = HuskyLangDatabase::new(None);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
mod husky_lord
struct A {}

main:
    let a = 1
"#
        .into(),
    );
    db.set_live_file_text(
        "haha/husky_lord.hsk".into(),
        r#"
struct B {}
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let package_scope_id = db.scope_to_id(Scope::new(
        db.string_to_word("haha".into()).ident().unwrap(),
        ScopeParent::Package(main_file_id),
        None,
    ));
    let subscope_table = db.subscope_table(package_scope_id).ok().unwrap();
    p!(subscope_table);
    assert_eq!(subscope_table.entries.len(), 3);
    assert_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn error1() {
    let mut db = HuskyLangDatabase::new(None);
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
stru ct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let package_scope_id = db.scope_to_id(Scope::new(
        db.string_to_word("haha".into()).ident().unwrap(),
        ScopeParent::Package(main_file_id),
        None,
    ));
    let subscope_table = db.subscope_table(package_scope_id).ok().unwrap();
    p!(subscope_table);
    assert_eq!(subscope_table.entries.len(), 1);
    assert_eq!(subscope_table.errors.len(), 1);
}
