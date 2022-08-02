use std::path::PathBuf;

xflags::xflags! {
    cmd husky-debugger-flags
    {
        optional --package-dir package_dir: PathBuf
        optional --warn-missing-linkage
        optional -v, --verbose
        optional --sample-id sample_id: String
        optional --mode mode: String
        optional --cdylib cdylib: String
        optional -c, --compiled
    }
}
