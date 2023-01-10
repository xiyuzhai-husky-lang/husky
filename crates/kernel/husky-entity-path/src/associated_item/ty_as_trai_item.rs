use super::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TypeAsTraitItemPath {
    pub ty_path: TypePath,
    pub trai_path: ModuleItemPath,
    pub ident: Identifier,
}
