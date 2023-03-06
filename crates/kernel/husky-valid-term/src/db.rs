use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_precise_term::PreciseTermDb;
use husky_precise_ty::PreciseTypeDb;
use salsa::DbWithJar;

pub trait ValidTermDb: DbWithJar<ValidTermJar> + PreciseTypeDb {}

impl<Db> ValidTermDb for Db where Db: DbWithJar<ValidTermJar> + PreciseTypeDb {}
