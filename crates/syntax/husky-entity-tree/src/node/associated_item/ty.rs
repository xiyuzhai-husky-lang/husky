use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeItemNodePath {
    pub path: TypeItemPath,
}

impl TypeItemNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TypeItemNode {
        todo!()
    }
}

impl From<TypeItemNodePath> for EntityNodePath {
    fn from(id: TypeItemNodePath) -> Self {
        EntityNodePath::AssociatedItem(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemNode {
    #[id]
    pub node_path: TypeItemNodePath,
    pub impl_block: TypeImplBlockNode,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub kind: TypeItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TypeItemNode {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.node_path(db).module_path(db)
    }
}
