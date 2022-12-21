use crate::*;
use husky_vfs::*;
use salsa::DbWithJar;

pub trait TomlTokenDb: DbWithJar<TomlTokenJar> + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn package_manifest_toml_token_sheet(&self, package: PackagePath)
        -> &VfsResult<TomlTokenSheet>;
}

impl<T> TomlTokenDb for T
where
    T: DbWithJar<TomlTokenJar> + VfsDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TomlTokenIter::new(self, input).collect()
    }

    fn package_manifest_toml_token_sheet(
        &self,
        package: PackagePath,
    ) -> &VfsResult<TomlTokenSheet> {
        package_manifest_toml_token_sheet(self, package)
    }
}

#[salsa::tracked(jar = TomlTokenJar, return_ref)]
pub(crate) fn package_manifest_toml_token_sheet(
    db: &dyn TomlTokenDb,
    package_path: PackagePath,
) -> VfsResult<TomlTokenSheet> {
    Ok(TomlTokenSheet::new(
        db.toml_tokenize(db.package_manifest_content(package_path)?),
    ))
}
