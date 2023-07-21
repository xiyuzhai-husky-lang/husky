use crate::*;

use salsa::DbWithJar;

pub trait DefnDb: DbWithJar<SynDefnJar> + DeclDb {}

impl<Db> DefnDb for Db where Db: DbWithJar<SynDefnJar> + DeclDb {}
