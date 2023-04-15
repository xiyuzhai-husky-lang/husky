use super::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + TermDb {}

impl<Db> TypeDb for Db where Db: salsa::DbWithJar<TypeJar> + TermDb {}
