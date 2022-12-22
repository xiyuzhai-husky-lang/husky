use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct AssociatedItemPath {
    pub parent_path: ModuleItemPath,
    pub ident: Identifier,
}
