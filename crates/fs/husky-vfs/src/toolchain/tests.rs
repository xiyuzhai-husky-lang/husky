use salsa::DebugWithDb;

use crate::*;

#[test]
fn toolchain_debug_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    expect_test::expect![[r#"
        Toolchain {
            [salsa id]: 0,
            data: Local {
                library_path: "/home/xiyuzhai/repos/husky/library",
            },
        }
    "#]]
    .assert_debug_eq(&toolchain.debug(&db));
}
