use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeItemPath {
    pub ty: TypePath,
    pub ident: Identifier,
    pub ty_item_kind: TypeItemKind,
}
