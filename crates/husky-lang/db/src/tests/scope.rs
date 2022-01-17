use scope::ScopeRoute;
use word::ReservedIdentifier;

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

    let main_file = db.intern_file("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.intern_word("haha".into()).custom().unwrap(),
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

    let main_file = db.intern_file("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.intern_word("haha".into()).custom().unwrap(),
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

    let main_file = db.intern_file("haha/main.hsk".into());
    let package = db.intern_scope(Scope::package(
        main_file,
        db.intern_word("haha".into()).custom().unwrap(),
    ));
    let subscope_table = db.subscope_table(package).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 1);
    should_eq!(subscope_table.errors.len(), 1);
}

#[test]
fn datasets() {
    let mut db = HuskyLangDatabase::new();
    let dataset_scope = db.intern_scope(Scope {
        route: ScopeRoute::Reserved {
            ident: ReservedIdentifier::Dataset,
        },
        generics: vec![],
    });
    let synthetic_scope = db.intern_scope(
        db.subscope(
            dataset_scope,
            db.intern_word("synthetic").custom().unwrap(),
            vec![],
        )
        .unwrap(),
    );
    let synthetic_trivial_scope = db
        .subscope(
            synthetic_scope,
            db.intern_word("trivial").custom().unwrap(),
            vec![],
        )
        .unwrap();
    let synthetic_trivial_scope = db.intern_scope(synthetic_trivial_scope);
    let _synthetic_trivial_real1d_scope = db.intern_scope(
        db.subscope(
            synthetic_trivial_scope,
            db.intern_word("trivial").custom().unwrap(),
            vec![],
        )
        .unwrap(),
    );
}
