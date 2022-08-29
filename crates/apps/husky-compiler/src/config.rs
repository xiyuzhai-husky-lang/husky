use crate::CompilerInstance;

impl CompilerInstance {
    pub(crate) fn sync_rust_code_verbose(&self) -> bool {
        true
    }

    pub(crate) fn rust_codegen_cache_diff_write_verbose(&self) -> bool {
        false
    }
}
