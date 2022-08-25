use crate::*;
use husky_package_semantics::Package;

impl CompilerInstance {
    pub(crate) fn clean_rust_gen_cache(&self, package_dir: &Path) {
        std::fs::remove_dir_all(package_dir.join("__rust_gen_cache__")).unwrap()
    }
}
