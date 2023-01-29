use super::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct TypeAsTraitItemPath {
    pub ty: TypePath,
    pub trai: TraitPath,
    pub ident: Identifier,
    pub ty_as_trai_item_kind: TraitItemKind,
}
