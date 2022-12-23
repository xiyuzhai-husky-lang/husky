use crate::*;
use salsa::DbWithJar;

pub trait SemanticTokenDb: DbWithJar<SemanticTokenJar> + TokenInferDb {}

impl<Db> SemanticTokenDb for Db where Db: DbWithJar<SemanticTokenJar> + TokenInferDb {}
