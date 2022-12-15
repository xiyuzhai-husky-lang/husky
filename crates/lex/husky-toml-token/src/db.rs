use crate::*;
use husky_package_path::{PackagePath, PackagePathDb};
use husky_source_path::{SourcePath, SourcePathData, SourcePathDb};
use husky_vfs::{VfsDb, VfsResult};
use salsa::DbWithJar;

pub trait TomlTokenDb: DbWithJar<TomlTokenJar> + SourcePathDb + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn toml_token_text(&self, path: SourcePath) -> &VfsResult<TomlTokenSheet>;

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenSheet>;
}

impl<T> TomlTokenDb for T
where
    T: DbWithJar<TomlTokenJar> + PackagePathDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TomlTokenIter::new(self, input).collect()
    }

    fn toml_token_text(&self, path: SourcePath) -> &VfsResult<TomlTokenSheet> {
        toml_token_text(self, path)
    }

    fn package_manifest_token_text(&self, package: PackagePath) -> &VfsResult<TomlTokenSheet> {
        self.toml_token_text(self.it_source_path(SourcePathData::CorgiToml(package)))
    }
}

#[salsa::tracked(jar = TomlTokenJar, return_ref)]
pub(crate) fn toml_token_text(db: &dyn TomlTokenDb, path: SourcePath) -> VfsResult<TomlTokenSheet> {
    Ok(TomlTokenSheet::new(db.toml_tokenize(db.source_text(path)?)))
}
