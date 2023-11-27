use vec_like::VecMapGetEntry;

use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct AttrSynNodePath {
    pub parent_syn_node_path: ItemSynNodePath,
    maybe_ambiguous_path: MaybeAmbiguousPath<AttrItemPath>,
}

impl AttrSynNodePath {
    fn new(
        parent_syn_node_path: ItemSynNodePath,
        path: AttrItemPath,
        registry: &mut ItemSynNodePathRegistry,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(
            db,
            parent_syn_node_path,
            registry.issue_maybe_ambiguous_path(path),
        )
    }

    pub fn path(self, db: &::salsa::Db) -> Option<AttrItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub(crate) fn syn_node(self, db: &::salsa::Db) -> AttrSynNode {
        attr_node(db, self)
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.maybe_ambiguous_path(db).path.ident(db)
    }
}

impl HasSynNodePath for AttrItemPath {
    type SynNodePath = AttrSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        AttrSynNodePath::new_inner(
            db,
            self.parent(db).syn_node_path(db),
            MaybeAmbiguousPath::from_path(self),
        )
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn attr_node(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> AttrSynNode {
    syn_node_path
        .parent_syn_node_path(db)
        .attr_syn_nodes(db)
        .get_entry(syn_node_path)
        .expect("todo")
        .1
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AttrSynNode {
    syn_node_path: AttrSynNodePath,
    ast_idx: AstIdx,
}

impl AttrSynNode {
    pub(crate) fn new(
        parent_path: ItemSynNodePath,
        path: AttrItemPath,
        ast_idx: AstIdx,
        registry: &mut ItemSynNodePathRegistry,
        db: &::salsa::Db,
    ) -> (AttrSynNodePath, Self) {
        let syn_node_path = AttrSynNodePath::new(parent_path, path, registry, db);
        (
            syn_node_path,
            AttrSynNode::new_inner(db, syn_node_path, ast_idx),
        )
    }
}
