use crate::*;
use husky_valid_term::RawTermDb;

pub trait ValidTermDb: salsa::DbWithJar<ValidTypeJar> + RawTermDb {}

impl<Db> ValidTermDb for Db where Db: salsa::DbWithJar<ValidTypeJar> + RawTermDb {}
