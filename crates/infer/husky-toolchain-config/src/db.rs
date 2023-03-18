use crate::*;
use husky_toolchain_config_ast::ToolchainConfigAstDb;

pub trait ToolchainConfigDb: salsa::DbWithJar<ToolchainConfigJar> + ToolchainConfigAstDb {}

impl<DB> ToolchainConfigDb for DB where
    DB: salsa::DbWithJar<ToolchainConfigJar> + ToolchainConfigAstDb
{
}
