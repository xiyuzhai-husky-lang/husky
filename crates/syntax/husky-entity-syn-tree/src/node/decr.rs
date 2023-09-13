use vec_like::VecMapGetEntry;

use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct DecrSynNodePath {
    pub parent_syn_node_path: ItemSynNodePath,
    maybe_ambiguous_path: MaybeAmbiguousPath<DecrPath>,
}

impl DecrSynNodePath {
    fn new(
        parent_syn_node_path: ItemSynNodePath,
        path: DecrPath,
        registry: &mut ItemSynNodePathRegistry,
        db: &dyn EntitySynTreeDb,
    ) -> Self {
        Self::new_inner(
            db,
            parent_syn_node_path,
            registry.issue_maybe_ambiguous_path(path),
        )
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<DecrPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> DecrSynNode {
        decr_node(db, self)
    }

    pub fn ident(self, db: &dyn EntitySynTreeDb) -> Ident {
        self.maybe_ambiguous_path(db).path.ident(db)
    }
}

impl<Db> HasModulePath<Db> for DecrSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        let db = entity_syn_tree_db(db);
        self.maybe_ambiguous_path(db).path.module_path(db)
    }
}

impl HasSynNodePath for DecrPath {
    type SynNodePath = DecrSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        DecrSynNodePath::new_inner(
            db,
            self.parent(db).syn_node_path(db),
            MaybeAmbiguousPath::from_path(self),
        )
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn decr_node(db: &dyn EntitySynTreeDb, syn_node_path: DecrSynNodePath) -> DecrSynNode {
    syn_node_path
        .parent_syn_node_path(db)
        .decrs(db)
        .get_entry(syn_node_path)
        .expect("todo")
        .1
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub(crate) struct DecrSynNode {
    #[id]
    pub syn_node_path: DecrSynNodePath,
    pub ast_idx: AstIdx,
}

impl DecrSynNode {
    pub(crate) fn new(
        parent_path: ItemSynNodePath,
        path: DecrPath,
        ast_idx: AstIdx,
        registry: &mut ItemSynNodePathRegistry,
        db: &dyn EntitySynTreeDb,
    ) -> (DecrSynNodePath, Self) {
        let syn_node_path = DecrSynNodePath::new(parent_path, path, registry, db);
        (
            syn_node_path,
            DecrSynNode::new_inner(db, syn_node_path, ast_idx),
        )
    }
}
