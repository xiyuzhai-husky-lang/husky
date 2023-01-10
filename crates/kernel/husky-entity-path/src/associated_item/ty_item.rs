use super::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TypeItemPath {
    pub ty_path: TypePath,
    pub ident: Identifier,
    pub ty_item_kind: TypeItemKind,
}
