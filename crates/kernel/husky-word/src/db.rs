use crate::*;

pub trait WordDb: salsa::DbWithJar<WordJar> {}

#[salsa::jar(db = WordDb)]
pub struct WordJar(vocabulary::vocabulary);
