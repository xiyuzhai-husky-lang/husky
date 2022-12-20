use crate::*;

use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_package_path::PackagePathJar;
use husky_token::TokenJar;
use husky_toolchain::*;
use husky_toolchain_infer::ToolchainInferJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    VfsJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ToolchainInferJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
