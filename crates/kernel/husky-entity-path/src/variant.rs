use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeVariantPath {
    pub path: TypePath,
    pub ident: Ident,
}

impl<Db> salsa::DisplayWithDb<Db> for TypeVariantPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}
