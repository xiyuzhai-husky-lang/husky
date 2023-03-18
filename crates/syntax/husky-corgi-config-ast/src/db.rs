use crate::*;

pub trait CorgiConfigAstDb: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb {}

impl<Db> CorgiConfigAstDb for Db where Db: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb {}
