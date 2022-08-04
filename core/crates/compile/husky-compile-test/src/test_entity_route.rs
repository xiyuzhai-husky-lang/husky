use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
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
    let pack = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subscope_table = db.subroute_table(pack).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 2);
    should_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn no_error_many_files() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
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
    let husky_lord_file = db.intern_file("haha/husky_lord.hsk".into());
    let package = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subroute_table = db.subroute_table(package).ok().unwrap();
    let husky_lord_route = db.subroute(package, db.intern_word("husky_lord").custom(), thin_vec![]);
    should_eq!(
        db.entity_source(husky_lord_route).unwrap(),
        EntitySource::Module {
            file: husky_lord_file
        }
    );
    should_eq!(subroute_table.entries.len(), 3);
    should_eq!(subroute_table.errors.len(), 0);
}
