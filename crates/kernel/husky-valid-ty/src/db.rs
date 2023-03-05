use crate::*;
use husky_valid_term::ValidTermDb;

pub trait ValidTypeDb: salsa::DbWithJar<ValidTypeJar> + ValidTermDb {}

impl<Db> ValidTypeDb for Db where Db: salsa::DbWithJar<ValidTypeJar> + ValidTermDb {}
