use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeNodePath {
    pub path: TypePath,
}

impl TypeNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, path)
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for TypePath {
    type NodePath = TypeNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeNodePath::new_inner(db, self)
    }
}

impl From<TypeNodePath> for EntityNodePath {
    fn from(id: TypeNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
