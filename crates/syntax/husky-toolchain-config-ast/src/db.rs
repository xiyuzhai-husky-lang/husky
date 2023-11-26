use crate::*;

pub trait ToolchainConfigAstDb: TomlAstDb {}

impl  ToolchainConfigAstDb for Db where Db: salsa::DbWithJar<TomlAstJar> + TomlAstDb {}
