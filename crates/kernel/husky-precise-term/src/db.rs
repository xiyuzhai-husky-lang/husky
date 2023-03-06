use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_raw_term::RawTermDb;
use salsa::DbWithJar;

pub trait RawTermDb: DbWithJar<RawTermJar> + RawTermDb {}

impl<Db> RawTermDb for Db where Db: DbWithJar<RawTermJar> + RawTermDb {}
