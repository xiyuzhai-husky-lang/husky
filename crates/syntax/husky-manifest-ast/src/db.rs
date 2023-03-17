use crate::*;
use husky_toml_ast::TomlAstDb;

pub trait ManifestAstDb: salsa::DbWithJar<ManifestAstJar> + TomlAstDb {}

impl<Db: ?Sized> ManifestAstDb for Db where Db: salsa::DbWithJar<ManifestAstJar> + TomlAstDb {}
