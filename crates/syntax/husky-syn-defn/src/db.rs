use crate::*;

use salsa::DbWithJar;

pub trait SynDefnDb: DbWithJar<SynDefnJar> + DeclDb {}

impl<Db> SynDefnDb for Db where Db: DbWithJar<SynDefnJar> + DeclDb {}
