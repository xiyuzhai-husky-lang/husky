use std::path::PathBuf;

xflags::xflags! {
    cmd husky-tracer-command
        required dir: PathBuf
    {
        optional -v, --verbose
        optional --sample-idx sample_idx: String
        optional --mode mode: String
        optional -c, --compile
    }
}
