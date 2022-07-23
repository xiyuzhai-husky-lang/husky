use std::path::PathBuf;

xflags::xflags! {
    cmd husky-compiler-flags
    required packages_dir: PathBuf {
    }
}
