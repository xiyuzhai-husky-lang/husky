use crate::*;

pub trait CHirDb: salsa::DbWithJar<CHirJar> {}

impl<Db> CHirDb for Db where Db: salsa::DbWithJar<CHirJar> {}
