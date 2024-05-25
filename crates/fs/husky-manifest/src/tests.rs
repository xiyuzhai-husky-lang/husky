pub(crate) use husky_vfs::test_utils::*;

#[salsa::db(
    husky_vfs::jar::VfsJar,
    husky_coword::jar::CowordJar,
    husky_toml_token::jar::TomlTokenJar,
    husky_toml_ast::jar::TomlAstJar,
    husky_manifest_ast::jar::ManifestAstJar,
    crate::Jar
)]
#[derive(Default)]
pub(crate) struct DB;
