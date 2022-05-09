use std::path::PathBuf;

xflags::xflags! {
    cmd husky-analyzer-printer
        required mode: String
        required dir: PathBuf
    {
        optional -v, --verbose
    }
}
