use crate::{dependency::rust_transpilation_dependencies, *};
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use is::Is;

pub trait TranspileToFsFull: Is<LinktimeTargetPath> {
    /// transpile the target crate and its dependencies
    fn transpile_to_fs_full(self, db: &dyn RustTranspilationDb);
}

impl TranspileToFsFull for LinktimeTargetPath {
    fn transpile_to_fs_full(self, db: &dyn RustTranspilationDb) {
        for dep in rust_transpilation_dependencies(self, db) {
            dep.transpile_to_fs(db)
        }
    }
}
