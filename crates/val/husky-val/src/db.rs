use crate::*;
use husky_linkage::db::LinkageDb;

pub trait ValDb: salsa::DbWithJar<ValJar> + LinkageDb {}

impl<Db> ValDb for Db where Db: salsa::DbWithJar<ValJar> + LinkageDb {}

#[salsa::jar(db = ValDb)]
pub struct ValJar(Val, ValDeps);
