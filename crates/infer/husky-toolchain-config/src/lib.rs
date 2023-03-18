mod db;

pub use db::*;

#[salsa::jar(db = ToolchainConfigDb)]
pub struct ToolchainConfigJar();
