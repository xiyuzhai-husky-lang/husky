use crate::*;
use salsa::DbWithJar;

pub trait IdentifierDb: DbWithJar<IdentifierJar> {
    fn identifier(&self, data: String) -> Identifier;
}

impl<T> IdentifierDb for T
where
    T: DbWithJar<IdentifierJar>,
{
    fn identifier(&self, data: String) -> Identifier {
        Identifier::new(self, data)
    }
}
