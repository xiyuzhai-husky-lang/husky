use crate::*;
use husky_coword::CowordJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;

#[salsa::db(CowordJar, VfsJar, TomlTokenJar, TomlAstJar, CorgiConfigAstJar)]
pub(crate) struct DB;
