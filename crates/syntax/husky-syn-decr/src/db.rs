use crate::*;
use husky_syn_decl::SynDeclDb;

pub trait DecrDb: salsa::DbWithJar<SynDecrJar> + SynDeclDb {}

impl<Db> DecrDb for Db where Db: salsa::DbWithJar<SynDecrJar> + SynDeclDb {}
