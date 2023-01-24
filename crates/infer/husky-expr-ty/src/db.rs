use crate::*;

pub trait ExprTypeDb: salsa::DbWithJar<ExprTypeJar> + TypeDb {}

impl<Db> ExprTypeDb for Db where Db: salsa::DbWithJar<ExprTypeJar> + TypeDb {}
