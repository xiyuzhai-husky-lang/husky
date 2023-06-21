use crate::*;

use salsa::DbWithJar;

pub trait DefnDb: DbWithJar<DefnJar> + DeclDb {}

impl<Db> DefnDb for Db where Db: DbWithJar<DefnJar> + DeclDb {}
