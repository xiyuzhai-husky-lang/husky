use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file = db.file_id("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.string_to_word("haha".into()).custom_ident().unwrap(),
    ));
    let subscope_table = db.subscope_table(package).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 2);
    should_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn no_error_many_files() {
    let mut db = HuskyLangDatabase::new();
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

    let main_file = db.file_id("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.string_to_word("haha".into()).custom_ident().unwrap(),
    ));
    let subscope_table = db.subscope_table(package).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 3);
    should_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn error1() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
stru ct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file = db.file_id("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.string_to_word("haha".into()).custom_ident().unwrap(),
    ));
    let subscope_table = db.subscope_table(package).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 1);
    should_eq!(subscope_table.errors.len(), 1);
}
