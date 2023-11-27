use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubmoduleSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
        self.data(db).maybe_ambiguous_path.unambiguous_path()
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a SubmoduleSynNode {
        match self.0.syn_node(db) {
            ItemSynNode::Submodule(data) => data,
            _ => unreachable!(),
        }
    }
}

impl SubmoduleSynNodePathData {
    pub(crate) fn syn_node(
        db: &::salsa::Db,
        syn_node_path: SubmoduleSynNodePath,
    ) -> SubmoduleSynNode {
        let module_path: ModulePath = todo!(); //= syn_node_path.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        match item_tree_sheet.major_item_node(syn_node_path.into()) {
            Some(ItemSynNode::Submodule(node)) => node,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn unambiguous_path(self, db: &::salsa::Db) -> Option<SubmodulePath> {
        self.syn_node_path.path(db)
    }
}
