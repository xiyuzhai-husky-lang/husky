use crate::*;
use husky_entity_tree::EntityTreeDb;

pub trait ExprDb: salsa::DbWithJar<ExprJar> + EntityTreeDb {}

impl<Db> ExprDb for Db where Db: salsa::DbWithJar<ExprJar> + EntityTreeDb {}
