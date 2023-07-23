use crate::*;

use salsa::DbWithJar;

pub trait SynDefnDb: DbWithJar<SynDefnJar> + SynDeclDb {}

impl<Db> SynDefnDb for Db where Db: DbWithJar<SynDefnJar> + SynDeclDb {}
