use crate::*;

pub trait DecrDb: salsa::DbWithJar<DecrJar> {}

impl<Db> DecrDb for Db where Db: salsa::DbWithJar<DecrJar> {}
