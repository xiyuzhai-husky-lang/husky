use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_precise_term::RawTermDb;
use husky_ty::PreciseTermDb;
use salsa::DbWithJar;

pub trait RawTermDb: DbWithJar<RawTermJar> + PreciseTermDb {}

impl<Db> RawTermDb for Db where Db: DbWithJar<RawTermJar> + PreciseTermDb {}
