use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
pub struct SubmoduleSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct SubmoduleSynNodePathData {
    maybe_ambiguous_path: MaybeAmbiguousPath<SubmodulePath>,
}

impl SubmoduleSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: SubmodulePath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Submodule(SubmoduleSynNodePathData {
                maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
            }),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> SubmoduleSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::Submodule(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> Option<SubmodulePath> {
        self.data(db).path()
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a SubmoduleSynNode {
        let module_path: ModulePath = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        match item_tree_sheet.major_item_node(self.into()) {
            Some(ItemSynNode::Submodule(node)) => node,
            _ => unreachable!(),
        }
    }
}

impl SubmoduleSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> SubmoduleSynNodePath {
        SubmoduleSynNodePath(id)
    }

    pub fn path(self) -> Option<SubmodulePath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.maybe_ambiguous_path.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        SubmoduleSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for SubmodulePath {
    type SynNodePath = SubmoduleSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        SubmoduleSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::Submodule(SubmoduleSynNodePathData {
                maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
            }),
        ))
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct SubmoduleSynNode {
    pub(crate) syn_node_path: SubmoduleSynNodePath,
    pub(crate) visibility: Scope,
    pub(crate) ast_idx: AstIdx,
    pub(crate) ident_token: IdentToken,
}

impl SubmoduleSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        submodule_path: SubmodulePath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> Self {
        Self {
            syn_node_path: SubmoduleSynNodePath::new(db, registry, submodule_path),
            visibility,
            ast_idx,
            ident_token,
        }
    }

    pub fn unambiguous_path(&self, db: &::salsa::Db) -> Option<SubmodulePath> {
        self.syn_node_path.path(db)
    }
}
