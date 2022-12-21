use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct ModuleItemVariantPath {
    pub module_item: ModuleItemPath,
    pub ident: Identifier,
}

impl<Db: EntityPathDb> salsa::DebugWithDb<Db> for ModuleItemVariantPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
