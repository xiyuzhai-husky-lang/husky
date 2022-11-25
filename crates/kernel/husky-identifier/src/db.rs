use crate::*;
use husky_word::WordDb;
use salsa::DbWithJar;

pub trait IdentifierDb: WordDb {
    fn it_ident_owned(&self, data: String) -> Identifier;

    fn it_ident_borrowed(&self, data: &str) -> Identifier;

    fn dt_ident(&self, data: Identifier) -> &str;
}

impl<T> IdentifierDb for T
where
    T: WordDb,
{
    fn it_ident_owned(&self, data: String) -> Identifier {
        Identifier::from_owned(self, data)
    }

    fn it_ident_borrowed(&self, data: &str) -> Identifier {
        Identifier::from_borrowed(self, data)
    }

    fn dt_ident(&self, ident: Identifier) -> &str {
        ident.data(self)
    }
}
