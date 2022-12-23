use crate::*;
use salsa::DbWithJar;

pub trait TokenInferDb: DbWithJar<TokenInferJar> + TokenDb {}

impl<Db> TokenInferDb for Db where Db: DbWithJar<TokenInferJar> + TokenDb {}
