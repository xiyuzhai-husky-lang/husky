use crate::*;
use husky_entity_path::EntityPathDb;

pub trait TermPreludeDb: salsa::DbWithJar<TermPreludeJar> + EntityPathDb {}

impl TermPreludeDb for Db where Db: salsa::DbWithJar<TermPreludeJar> + EntityPathDb {}
