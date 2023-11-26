use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubmoduleSynNodePath(ItemSynNodePathId);

#[derive(Debug, PartialEq, Eq)]
pub struct SubmoduleSynNodePathData {
    maybe_ambiguous_path: MaybeAmbiguousPath<SubmodulePath>,
}

impl SubmoduleSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: SubmodulePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<SubmodulePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub(crate) fn syn_node_data(self, db: &::salsa::Db) -> SubmoduleSynNodeData {
        match self.0.syn_node_data(db) {
            ItemSynNodeData::Submodule(data) => data,
            _ => unreachable!(),
        }
    }
}

impl SubmoduleSynNodePathData {
    pub(crate) fn syn_node(
        db: &::salsa::Db,
        syn_node_path: SubmoduleSynNodePath,
    ) -> SubmoduleSynNodeData {
        let module_path: ModulePath = todo!(); //= syn_node_path.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        match item_tree_sheet.major_item_node(syn_node_path.into()) {
            Some(ItemSynNodeData::Submodule(node)) => node,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct SubmoduleSynNodeData {
    syn_node_path: SubmoduleSynNodePath,
    visibility: Scope,
    ast_idx: AstIdx,
    ident_token: IdentToken,
}

impl SubmoduleSynNodeData {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        submodule_path: SubmodulePath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> Self {
        Self::new_inner(
            db,
            SubmoduleSynNodePath::new(db, registry, submodule_path),
            visibility,
            ast_idx,
            ident_token,
        )
    }

    pub fn unambiguous_path(self, db: &::salsa::Db) -> Option<SubmodulePath> {
        self.syn_node_path(db).path(db)
    }
}
