use crate::*;
use husky_vfs::*;
use salsa::DbWithJar;

pub trait TomlTokenDb: DbWithJar<TomlTokenJar> + VfsDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn toml_token_sheet(&self, path: DiffPath) -> VfsResult<Option<&TomlTokenSheet>>;
}

impl<T> TomlTokenDb for T
where
    T: DbWithJar<TomlTokenJar> + VfsDb + VfsDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TomlTokenIter::new(self, input).collect()
    }

    fn toml_token_sheet(&self, path: DiffPath) -> VfsResult<Option<&TomlTokenSheet>> {
        match toml_token_sheet(self, path) {
            Ok(Some(sheet)) => Ok(Some(sheet)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.clone()),
        }
    }
}

#[salsa::tracked(jar = TomlTokenJar, return_ref)]
pub(crate) fn toml_token_sheet(
    db: &dyn TomlTokenDb,
    path: DiffPath,
) -> VfsResult<Option<TomlTokenSheet>> {
    Ok(path
        .text(db)?
        .map(|text| TomlTokenSheet::new(db.toml_tokenize(text))))
}
