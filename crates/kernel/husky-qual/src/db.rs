use crate::*;

pub trait QualDb: salsa::DbWithJar<QualJar> {}

impl<Db> QualDb for Db where Db: salsa::DbWithJar<QualJar> {}
