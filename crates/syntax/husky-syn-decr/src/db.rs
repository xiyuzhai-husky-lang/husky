use crate::*;
use husky_syn_decl::DeclDb;

pub trait DecrDb: salsa::DbWithJar<SynDecrJar> + DeclDb {}

impl<Db> DecrDb for Db where Db: salsa::DbWithJar<SynDecrJar> + DeclDb {}
