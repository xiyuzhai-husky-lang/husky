use crate::*;
use husky_vfs::{error::VfsResult, *};

pub trait TomlTokenDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;

    fn toml_token_sheet(&self, path: VirtualPath) -> VfsResult<Option<&TomlTokenSheet>>;
}

impl TomlTokenDb for ::salsa::Db {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TomlTokenIter::new(self, input).collect()
    }

    fn toml_token_sheet(&self, path: VirtualPath) -> VfsResult<Option<&TomlTokenSheet>> {
        match toml_token_sheet(self, path) {
            Ok(Some(sheet)) => Ok(Some(sheet)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.clone()),
        }
    }
}

#[salsa::tracked(jar = TomlTokenJar, return_ref)]
pub(crate) fn toml_token_sheet(
    db: &::salsa::Db,
    path: VirtualPath,
) -> VfsResult<Option<TomlTokenSheet>> {
    Ok(path
        .text(db)?
        .map(|text| TomlTokenSheet::new(db.toml_tokenize(text))))
}
