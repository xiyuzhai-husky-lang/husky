use std::path::PathBuf;

xflags::xflags! {
    cmd husky-lang-debugger-command
        required path: PathBuf
    {
        optional -v, --verbose
        optional --input-id input_id: String
        optional --mode mode: String
        optional -c, --compile
    }
}
