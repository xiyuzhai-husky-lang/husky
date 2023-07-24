pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_vfs::VfsJar;
use salsa::{Database, Storage};
use with_db::WithDb;

#[salsa::db(CowordJar, VfsJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}

#[test]
fn visibility_partial_ord_works() {
    use Scope::*;

    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();
    assert!(Pub.with_db(&db) > PubUnder(path_menu.core_num().inner()).with_db(&db));
    assert!(
        !(PubUnder(path_menu.core_prelude().inner()).with_db(&db)
            > PubUnder(path_menu.core_num().inner()).with_db(&db))
    );
    assert!(Pub.with_db(&db) > Private(path_menu.core_num().inner()).with_db(&db));
    assert!(Pub.with_db(&db) >= Pub.with_db(&db));
    // equals
    assert_eq!(Pub.with_db(&db), Pub.with_db(&db));
    assert_eq!(
        Private(path_menu.core_num().inner()).with_db(&db),
        Private(path_menu.core_num().inner()).with_db(&db)
    );
    assert_eq!(
        PubUnder(path_menu.core_prelude().inner()).with_db(&db),
        PubUnder(path_menu.core_prelude().inner()).with_db(&db)
    );
    // not equals
    assert_ne!(
        Pub.with_db(&db),
        Private(path_menu.core_num().inner()).with_db(&db)
    );
    assert_ne!(
        Private(path_menu.core_num().inner()).with_db(&db),
        Pub.with_db(&db)
    );
    assert_ne!(
        Private(path_menu.core_num().inner()).with_db(&db),
        PubUnder(path_menu.core_num().inner()).with_db(&db)
    );
    assert_ne!(
        PubUnder(path_menu.core_prelude().inner()).with_db(&db),
        PubUnder(path_menu.core_num().inner()).with_db(&db)
    );
}
