use crate::*;
use salsa::DbWithJar;

pub trait IdentifierDb: DbWithJar<IdentifierJar> {
    fn identifier(&self, data: String) -> Identifier;

    fn it_ident(&self, ident: &str) -> Identifier;

    fn dt_ident(&self, ident: Identifier) -> &str;
}

impl<T> IdentifierDb for T
where
    T: DbWithJar<IdentifierJar>,
{
    fn identifier(&self, data: String) -> Identifier {
        Identifier::new(self, data)
    }

    fn it_ident(&self, ident: &str) -> Identifier {
        Identifier::new(self, ident.to_string())
    }

    fn dt_ident(&self, ident: Identifier) -> &str {
        ident.data(self)
    }
}
