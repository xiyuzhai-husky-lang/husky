use std::path::PathBuf;

xflags::xflags! {
    cmd husky-compiler-flags
    {
        required --src src: PathBuf
        required --dst src: PathBuf
        required --rel-husky-dir rel_husky_dir: PathBuf
    }
}
