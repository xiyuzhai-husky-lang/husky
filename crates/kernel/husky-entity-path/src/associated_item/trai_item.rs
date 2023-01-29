use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TraitItemPath {
    pub trai_path: TraitPath,
    pub ident: Identifier,
}
