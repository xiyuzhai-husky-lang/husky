use crate::*;
use husky_package_path::PackagePath;
use husky_vfs::VfsResult;

#[salsa::tracked(jar = TomlTokenizeJar, return_ref)]
pub(crate) fn package_manifest_toml_token_sheet(
    db: &dyn TomlTokenizeDb,
    package: PackagePath,
) -> VfsResult<TomlTokens> {
    let file = db.package_manifest_toml_file(package).unwrap();
    let file_content = db.file_content(file);
    Ok(TomlTokens(db.toml_tokenize(file_content)))
}

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlTokens(Vec<TomlToken>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TomlTokenGroup(std::ops::Range<usize>);

impl TomlTokenGroup {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        Self(range)
    }

    pub fn first(&self, sheet: &TomlTokens) -> &TomlToken {
        todo!()
    }
}

impl std::ops::Index<TomlTokenGroup> for TomlTokens {
    type Output = [TomlToken];

    fn index(&self, index: TomlTokenGroup) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::Deref for TomlTokenGroup {
    type Target = std::ops::Range<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
