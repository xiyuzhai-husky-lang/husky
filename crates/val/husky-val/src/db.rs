use crate::*;

pub trait ValDb: salsa::DbWithJar<ValJar> {}

impl<Db> ValDb for Db where Db: salsa::DbWithJar<ValJar> {}

#[salsa::jar(db = ValDb)]
pub struct ValJar(Val, ValDeps);
