mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

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
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: ModuleItemPath,
    ) -> Self {
        match path {
            ModuleItemPath::Type(path) => TypeNodePath::new(db, registry, path).into(),
            ModuleItemPath::Trait(path) => TraitNodePath::new(db, registry, path).into(),
            ModuleItemPath::Fugitive(path) => FugitiveNodePath::new(db, registry, path).into(),
        }
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> ModuleItemPath {
        todo!()
        // match self {
        //     ModuleItemNodePath::Trait(id) => id.path(db).into(),
        //     ModuleItemNodePath::Type(id) => id.path(db).into(),
        //     ModuleItemNodePath::Fugitive(id) => id.path(db).into(),
        // }
    }

    pub fn ident(self, db: &dyn EntityTreeDb) -> Ident {
        self.path(db).ident(db)
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
            ModuleItemPath::Fugitive(path) => path.node_path(db).into(),
        }
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct ModuleItemNode {
    #[id]
    pub node_path: ModuleItemNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}
