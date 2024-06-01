use crate::*;
use husky_coword::jar::CowordJar;
use husky_toml_token::jar::TomlTokenJar;

#[salsa::db(
    CowordJar,
    husky_vfs::jar::VfsJar,
    TomlTokenJar,
    TomlAstJar,
    CorgiConfigAstJar
)]
pub(crate) struct DB;
