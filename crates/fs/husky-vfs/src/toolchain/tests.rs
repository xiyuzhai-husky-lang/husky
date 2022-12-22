use salsa::DebugWithDb;

use crate::*;

#[test]
fn toolchain_debug_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain();
    expect_test::expect![[r#"
        Toolchain {
            [salsa id]: 0,
            data: Local {
                library_path: DiffPath {
                    [salsa id]: 0,
                    data: DiffPathBuf(
                        "../../../library",
                    ),
                },
            },
        }
    "#]]
    .assert_debug_eq(&toolchain.debug(&db));
}
