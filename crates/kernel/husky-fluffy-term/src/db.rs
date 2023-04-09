use crate::*;

pub trait FluffyTermDb: TermDb + salsa::DbWithJar<FluffyTermJar> {}

impl<Db> FluffyTermDb for Db where Db: TermDb + salsa::DbWithJar<FluffyTermJar> {}
