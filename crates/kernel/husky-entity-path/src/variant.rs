use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct VariantPath {
    pub path: TypePath,
    pub ident: Identifier,
}

impl<Db: EntityPathDb> salsa::DebugWithDb<Db> for VariantPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
