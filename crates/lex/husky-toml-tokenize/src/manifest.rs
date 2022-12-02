use crate::*;
use husky_package_path::PackagePath;
use husky_source_path::SourcePath;
use husky_toml_token_text::TomlTokenText;
use husky_vfs::VfsResult;

#[salsa::tracked(jar = TomlTokenizeJar, return_ref)]
pub(crate) fn toml_token_text(
    db: &dyn TomlTokenizeDb,
    path: SourcePath,
) -> VfsResult<TomlTokenText> {
    let file = db.file(path).unwrap();
    let file_content = db.file_content(file);
    Ok(TomlTokenText::new(db.toml_tokenize(file_content)))
}
