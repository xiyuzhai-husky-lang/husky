use std::path::PathBuf;

xflags::xflags! {
    cmd husky-analyzer-tester
        required mode: String
        required dir: PathBuf
    {
        optional -v, --verbose
    }
}
