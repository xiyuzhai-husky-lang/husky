use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeItemNodeId {
    pub path: TypeItemPath,
}

impl TypeItemNodeId {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }

    pub fn impl_block(self, db: &dyn EntityTreeDb) -> TypeImplBlockNodeId {
        self.path(db).impl_block(db).node_id(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TypeItemNode {
        todo!()
    }
}

impl From<TypeItemNodeId> for EntityNodeId {
    fn from(id: TypeItemNodeId) -> Self {
        EntityNodeId::AssociatedItem(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemNode {
    #[id]
    pub node_id: TypeItemNodeId,
    pub impl_block: TypeImplBlockNode,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub kind: TypeItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TypeItemNode {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.node_id(db).module_path(db)
    }
}
