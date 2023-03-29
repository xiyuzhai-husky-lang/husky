use crate::*;
use husky_decl::DeclDb;

pub trait DecrDb: salsa::DbWithJar<DecrJar> + DeclDb {}

impl<Db> DecrDb for Db where Db: salsa::DbWithJar<DecrJar> + DeclDb {}
