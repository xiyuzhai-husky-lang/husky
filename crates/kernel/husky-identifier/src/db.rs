use crate::*;
use salsa::DbWithJar;

pub trait IdentifierDb: DbWithJar<IdentifierJar> {}

impl<T> IdentifierDb for T where T: DbWithJar<IdentifierJar> {}
