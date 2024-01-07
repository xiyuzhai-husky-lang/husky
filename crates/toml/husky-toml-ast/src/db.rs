use crate::*;

use husky_vfs::error::{VfsError, VfsResult};

pub trait TomlAstDb {
    fn package_manifest_toml_ast_sheet(&self, path: PackagePath) -> VfsResult<&TomlAstSheet>;
    fn toml_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&TomlAstSheet>>;
}

impl TomlAstDb for ::salsa::Db {
    fn package_manifest_toml_ast_sheet(&self, path: PackagePath) -> VfsResult<&TomlAstSheet> {
        let path = path.manifest_path(self)?.path();
        self.toml_ast_sheet(path)?
            .ok_or(VfsError::FileNotExists(path))
    }
    fn toml_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&TomlAstSheet>> {
        toml_ast_sheet(self, path)
    }
}
