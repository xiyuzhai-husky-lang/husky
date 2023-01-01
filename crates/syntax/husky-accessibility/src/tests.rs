use husky_vfs::{VfsJar, VfsTestSupport};
use husky_word::WordJar;
use salsa::{Database, Storage};
use with_db::WithDb;

use crate::*;

#[salsa::db(WordJar, VfsJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}

#[test]
fn accessibility_partial_ord_works() {
    use Accessibility::*;

    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();
    assert!(Public.with_db(&db) > PublicUnder(path_menu.core_num()).with_db(&db));
    assert!(
        !(PublicUnder(path_menu.core_prelude()).with_db(&db)
            > PublicUnder(path_menu.core_num()).with_db(&db))
    );
    assert!(Public.with_db(&db) > Private.with_db(&db));
    assert!(Public.with_db(&db) >= Public.with_db(&db));
    // equals
    assert_eq!(Public.with_db(&db), Public.with_db(&db));
    assert_eq!(Private.with_db(&db), Private.with_db(&db));
    assert_eq!(
        PublicUnder(path_menu.core_prelude()).with_db(&db),
        PublicUnder(path_menu.core_prelude()).with_db(&db)
    );
    // not equals
    assert_ne!(Public.with_db(&db), Private.with_db(&db));
    assert_ne!(Private.with_db(&db), Public.with_db(&db));
    assert_ne!(
        Private.with_db(&db),
        PublicUnder(path_menu.core_num()).with_db(&db)
    );
    assert_ne!(
        PublicUnder(path_menu.core_prelude()).with_db(&db),
        PublicUnder(path_menu.core_num()).with_db(&db)
    );
}
