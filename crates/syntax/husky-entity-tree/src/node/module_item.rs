use super::*;
use husky_entity_path::ModuleItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum ModuleItemNodePath {
    Trait(TraitNodePath),
    Type(TypeNodePath),
    Fugitive(FugitiveNodePath),
}

impl ModuleItemNodePath {
    pub fn path(self, db: &dyn EntityTreeDb) -> ModuleItemPath {
        match self {
            ModuleItemNodePath::Trait(id) => id.path(db).into(),
            ModuleItemNodePath::Type(id) => id.path(db).into(),
            ModuleItemNodePath::Fugitive(id) => id.path(db).into(),
        }
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for ModuleItemPath {
    type NodePath = ModuleItemNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        match self {
            ModuleItemPath::Type(path) => path.node_path(db).into(),
            ModuleItemPath::Trait(path) => path.node_path(db).into(),
            ModuleItemPath::Form(path) => path.node_path(db).into(),
        }
    }
}

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitNodePath {
    pub path: TraitPath,
}

impl TraitNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for TraitPath {
    type NodePath = TraitNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TraitNodePath::new(db, self)
    }
}

impl From<TraitNodePath> for EntityNodePath {
    fn from(id: TraitNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeNodePath {
    pub path: TypePath,
}

impl TypeNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for TypePath {
    type NodePath = TypeNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeNodePath::new(db, self)
    }
}

impl From<TypeNodePath> for EntityNodePath {
    fn from(id: TypeNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct FugitiveNodePath {
    pub path: FugitivePath,
}

impl FugitiveNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for FugitivePath {
    type NodePath = FugitiveNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        FugitiveNodePath::new(db, self)
    }
}

impl From<FugitiveNodePath> for EntityNodePath {
    fn from(id: FugitiveNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
