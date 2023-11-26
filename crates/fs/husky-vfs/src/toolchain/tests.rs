use salsa::DebugWithDb;

use crate::*;

#[test]
fn toolchain_debug_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    expect_test::expect![[r#"
        Toolchain {
            [salsa id]: 0,
            data: ToolchainData::Local {
                library_path: VirtualPath {
                    _data: RelPathBuf(
                        "../../../library",
                    ),
                },
            },
        }
    "#]]
    .assert_debug_eq(&toolchain.debug(db));
}
