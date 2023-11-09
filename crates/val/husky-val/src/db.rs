use crate::*;
use husky_linkage_path::db::LinkagePathDb;

pub trait ValDb: salsa::DbWithJar<ValJar> + LinkagePathDb {}

impl<Db> ValDb for Db where Db: salsa::DbWithJar<ValJar> + LinkagePathDb {}

#[salsa::jar(db = ValDb)]
pub struct ValJar(Val, ValDeps);
