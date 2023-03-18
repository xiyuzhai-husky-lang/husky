use crate::*;

pub trait ToolchainConfigAstDb: TomlAstDb {}

impl<Db> ToolchainConfigAstDb for Db where Db: salsa::DbWithJar<TomlAstJar> + TomlAstDb {}
