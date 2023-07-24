use crate::*;
use husky_item_path::EntityPathDb;

pub trait TermPreludeDb: salsa::DbWithJar<TermPreludeJar> + EntityPathDb {}

impl<Db> TermPreludeDb for Db where Db: salsa::DbWithJar<TermPreludeJar> + EntityPathDb {}
