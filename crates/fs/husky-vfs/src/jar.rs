use crate::*;

#[salsa::jar(db = VfsDb)]
pub struct VfsJar(
    PackagePath,
    CratePath,
    ModulePath,
    apparent_ancestry,
    path_menu,
    PathBufItd,
    File,
    package_dir,
    module_absolute_path,
    package_manifest_file,
    module_file,
    // toolchain
    Toolchain,
    PublishedToolchain,
    published_toolchain_library_path,
);

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        todo!()
        // &self.0
    }

    pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
        todo!()
        // self.0.set_watcher(watcher)
    }
}
