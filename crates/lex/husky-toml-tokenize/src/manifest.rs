use crate::*;
use husky_package_path::PackagePath;
use husky_vfs::VfsResult;

#[salsa::tracked(jar = TomlTokenizeJar, return_ref)]
pub(crate) fn package_manifest_toml_tokens(
    db: &dyn TomlTokenizeDb,
    package: PackagePath,
) -> VfsResult<Vec<TomlToken>> {
    let file = db.package_manifest_toml_file(package).unwrap();
    let file_content = db.file_content(file);
    Ok(db.toml_tokenize(file_content))
}
