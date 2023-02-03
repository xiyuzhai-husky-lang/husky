use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct VariantPath {
    pub path: TypePath,
    pub ident: Identifier,
}

impl<Db> salsa::DisplayWithDb<Db> for VariantPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
