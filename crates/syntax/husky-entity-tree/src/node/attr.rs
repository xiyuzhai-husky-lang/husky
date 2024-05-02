use super::*;
use husky_entity_path::path::attr::AttrItemPath;
use vec_like::VecMapGetEntry;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityTreeJar)]
#[salsa::deref_id]
pub struct AttrSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AttrSynNodePathData {
    pub parent_syn_node_path: ItemSynNodePath,
    /// if the parent doesn't admit a maybe ambiguous item path, this is none
    attr_item_path_result: Result<AttrItemPath, (Ident, u8)>,
}

impl AttrSynNodePath {
    fn new(
        parent_syn_node_path: ItemSynNodePath,
        attr_item_path_result: Result<AttrItemPath, (Ident, u8)>,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Attr(AttrSynNodePathData {
                parent_syn_node_path,
                attr_item_path_result,
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

    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<AttrItemPath> {
        Some(match self.0.unambiguous_item_path(db)? {
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
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> AttrSynNodePath {
        AttrSynNodePath(id)
    }

    pub fn unambiguous_item_path(self) -> Option<AttrItemPath> {
        self.attr_item_path_result.ok()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.parent_syn_node_path.module_path(db)
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        match self.attr_item_path_result {
            Ok(attr_item_path) => attr_item_path.ident(db),
            Err((ident, _)) => ident,
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        AttrSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for AttrItemPath {
    type SynNodePath = AttrSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        AttrSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Attr(AttrSynNodePathData {
                parent_syn_node_path: self.parent(db).syn_node_path(db),
                attr_item_path_result: Ok(self),
            }),
        ))
    }
}

// #[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct AttrSynNode {
    syn_node_path: AttrSynNodePath,
    ast_idx: AstIdx,
}

/// # constructor
impl AttrSynNode {
    pub(crate) fn new(
        parent_path: ItemSynNodePath,
        attr_item_path_result: Result<AttrItemPath, (Ident, u8)>,
        ast_idx: AstIdx,
        db: &::salsa::Db,
    ) -> (AttrSynNodePath, Self) {
        let syn_node_path = AttrSynNodePath::new(parent_path, attr_item_path_result, db);
        (
            syn_node_path,
            AttrSynNode {
                syn_node_path,
                ast_idx,
            },
        )
    }
}

/// # getters
impl AttrSynNode {
    pub(crate) fn syn_node_path(&self) -> AttrSynNodePath {
        self.syn_node_path
    }
}

pub trait HasAttrPaths: Copy {
    type AttrPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath];
}

impl HasAttrPaths for ItemPathId {
    type AttrPath = AttrItemPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath] {
        item_attr_paths(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn item_attr_paths(db: &::salsa::Db, item_path_id: ItemPathId) -> SmallVec<[AttrItemPath; 2]> {
    item_path_id
        .item_path(db)
        .syn_node_path(db)
        .attr_syn_nodes(db)
        .iter()
        .filter_map(|(attr_syn_node_path, _)| attr_syn_node_path.unambiguous_item_path(db))
        .collect()
}

impl ItemSynNodePathId {
    pub(crate) fn attr_syn_nodes(self, db: &::salsa::Db) -> &[(AttrSynNodePath, AttrSynNode)] {
        item_attr_syn_nodes(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn item_attr_syn_nodes(
    db: &::salsa::Db,
    item_syn_node_path_id: ItemSynNodePathId,
) -> SmallVec<[(AttrSynNodePath, AttrSynNode); 2]> {
    let ast_sheet = item_syn_node_path_id.module_path(db).ast_sheet(db);
    let syn_node_path = item_syn_node_path_id.syn_node_path(db);
    match syn_node_path {
        ItemSynNodePath::Attr(_, _) => return smallvec![],
        _ => (),
    }
    let Some(ast_idx) = item_syn_node_path_id.opt_ast_idx(db) else {
        return smallvec![];
    };
    ast_sheet.procure_attrs(
        syn_node_path.unambiguous_item_path(db),
        ast_idx,
        move |attr_ast_idx, _, path| AttrSynNode::new(syn_node_path, path, attr_ast_idx, db),
        db,
    )
}
