use crate::*;
use husky_term_prelude::TermPreludeDb;
use salsa::DbWithJar;

pub trait TokenDataDb: DbWithJar<TokenDataJar> + TermPreludeDb {}

impl<T> TokenDataDb for T where T: DbWithJar<TokenDataJar> + TermPreludeDb {}

#[salsa::jar(db = TokenDataDb)]
pub struct TokenDataJar(UnspecifiedFloatLiteral);
