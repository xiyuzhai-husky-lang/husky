use std::path::PathBuf;

xflags::xflags! {
    cmd husky-compiler-flags
        required dir: PathBuf
    {}
}
