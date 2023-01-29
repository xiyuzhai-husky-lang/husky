use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct VariantPath {
    pub path: TypePath,
    pub ident: Identifier,
}
