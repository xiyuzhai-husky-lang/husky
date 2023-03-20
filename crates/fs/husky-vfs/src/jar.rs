use crate::*;

#[salsa::jar(db = VfsDb)]
pub struct VfsJar(
    VfsCache,
    PackagePath,
    CratePath,
    ModulePath,
    module_ancestry,
    vfs_path_menu,
    DiffPath,
    File,
    NotebookSection,
    NotebookSectionId,
    NotebookSubsection,
    NotebookSubsectionId,
    NotebookAst,
    package_dir,
    package_manifest_path,
    module_diff_path,
    package_manifest_file,
    module_file,
    Toolchain,
    PublishedToolchain,
    published_toolchain_library_path,
    current_toolchain,
);

impl salsa::storage::IngredientsFor for VfsCache {
    type Jar = VfsJar;

    type Ingredients = Self;

    fn create_ingredients<DB>(_routes: &mut salsa::routes::Routes<DB>) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        Default::default()
    }
}

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.0
    }

    pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
        self.0.set_watcher(watcher)
    }
}
