use super::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TypeItemPath {
    pub ty_path: TypePath,
    pub ident: Identifier,
}

impl Into<EntityPath> for TypeItemPath {
    fn into(self) -> EntityPath {
        EntityPath::AssociatedItem(self.into())
    }
}
