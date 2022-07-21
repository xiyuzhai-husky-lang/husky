use crate::*;
use husky_package_semantics::Package;

impl CompilerInstance {
    pub fn get_rust_dir(&self, package: &Package) -> PathBuf {
        let dashed_name = snake_to_dash(&package.ident);
        let rust_dir: PathBuf = self.dst.join(dashed_name);
        mkdir(&rust_dir);
        rust_dir
    }

    pub fn get_husky_code_snapshot_dir(&self, package: &Package) -> PathBuf {
        let rust_dir = self.get_rust_dir(package);
        assert!(rust_dir.exists());
        let snapshot_dir = get_or_create_child_dir(&rust_dir, "snapshot");
        get_or_create_child_dir(&snapshot_dir, &snake_to_dash(&package.ident))
    }
}
