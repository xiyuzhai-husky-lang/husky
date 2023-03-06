use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_raw_term::RawTermDb;
use salsa::DbWithJar;

pub trait PreciseTermDb: DbWithJar<PreciseTermJar> + RawTypeDb {}

impl<Db> PreciseTermDb for Db where Db: DbWithJar<PreciseTermJar> + RawTypeDb {}
