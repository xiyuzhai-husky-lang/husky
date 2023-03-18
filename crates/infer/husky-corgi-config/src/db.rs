use crate::*;
use husky_corgi_config_ast::CorgiConfigAstDb;

pub trait CorgiConfigDb: salsa::DbWithJar<CorgiConfigJar> + CorgiConfigAstDb {}

impl<DB> CorgiConfigDb for DB where DB: salsa::DbWithJar<CorgiConfigJar> + CorgiConfigAstDb {}
