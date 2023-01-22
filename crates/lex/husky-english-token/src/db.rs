use crate::*;
use husky_vfs::*;
use salsa::DbWithJar;

pub trait EnglishTokenDb: DbWithJar<EnglishTokenJar> + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<EnglishToken>;

    fn package_manifest_toml_token_sheet(
        &self,
        package: PackagePath,
    ) -> &VfsResult<EnglishTokenSheet>;
}

impl<T> EnglishTokenDb for T
where
    T: DbWithJar<EnglishTokenJar> + VfsDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<EnglishToken> {
        EnglishTokenIter::new(self, input).collect()
    }

    fn package_manifest_toml_token_sheet(
        &self,
        package: PackagePath,
    ) -> &VfsResult<EnglishTokenSheet> {
        package_manifest_toml_token_sheet(self, package)
    }
}

#[salsa::tracked(jar = EnglishTokenJar, return_ref)]
pub(crate) fn package_manifest_toml_token_sheet(
    db: &dyn EnglishTokenDb,
    package_path: PackagePath,
) -> VfsResult<EnglishTokenSheet> {
    Ok(EnglishTokenSheet::new(
        db.toml_tokenize(db.package_manifest_content(package_path)?),
    ))
}
