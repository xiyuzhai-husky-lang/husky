use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeVariantNodePath {
    path: TypeVariantPath,
}

impl TypeVariantNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for TypeVariantPath {
    type NodePath = TypeVariantNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeVariantNodePath::new(db, self)
    }
}
