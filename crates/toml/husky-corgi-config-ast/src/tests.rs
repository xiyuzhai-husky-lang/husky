use crate::*;
use husky_coword::jar::CowordJar;
use husky_toml_token::jar::TomlTokenJar;
use husky_vfs::*;

#[salsa::db(CowordJar, VfsJar, TomlTokenJar, TomlAstJar, CorgiConfigAstJar)]
pub(crate) struct DB;
