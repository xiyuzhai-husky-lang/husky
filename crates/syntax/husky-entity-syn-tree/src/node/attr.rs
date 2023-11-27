use vec_like::VecMapGetEntry;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntitySynTreeJar)]
#[salsa::deref_id]
pub struct AttrSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AttrSynNodePathData {
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
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Attr(AttrSynNodePathData {
                parent_syn_node_path,
                maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
            }),
        ))
    }

    pub fn data(self, db: &salsa::Db) -> AttrSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::Attr(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent_syn_node_path(self, db: &::salsa::Db) -> ItemSynNodePath {
        self.data(db).parent_syn_node_path
    }

    pub fn ident(self, db: &salsa::Db) -> Ident {
        self.data(db).ident(db)
    }

    pub fn path(self, db: &::salsa::Db) -> Option<AttrItemPath> {
        Some(match self.0.path(db)? {
            ItemPath::Attr(_, path) => path,
            _ => unreachable!(),
        })
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a AttrSynNode {
        &self
            .data(db)
            .parent_syn_node_path
            .attr_syn_nodes(db)
            .get_entry(self)
            .expect("todo")
            .1
    }
}

impl AttrSynNodePathData {
    pub fn path(self, db: &::salsa::Db) -> Option<AttrItemPath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.maybe_ambiguous_path.path.ident(db)
    }
}

impl HasSynNodePath for AttrItemPath {
    type SynNodePath = AttrSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        AttrSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Attr(AttrSynNodePathData {
                parent_syn_node_path: self.parent(db).syn_node_path(db),
                maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
            }),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
            AttrSynNode {
                syn_node_path,
                ast_idx,
            },
        )
    }
}
