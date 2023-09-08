use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct SubmoduleSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<SubmodulePath>,
}

impl SubmoduleSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        path: SubmodulePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<SubmodulePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> SubmoduleSynNode {
        submodule_syn_node(db, self)
    }
}

impl<Db> HasModulePath<Db> for SubmoduleSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        let db = entity_syn_tree_db(db);
        self.maybe_ambiguous_path(db)
            .path
            .parent(db)
            .expect("non root")
    }
}

impl HasSynNodePath for SubmodulePath {
    type SynNodePath = SubmoduleSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        SubmoduleSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct SubmoduleSynNode {
    #[id]
    pub syn_node_path: SubmoduleSynNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

impl SubmoduleSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
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

    pub fn unambiguous_path(self, db: &dyn EntitySynTreeDb) -> Option<SubmodulePath> {
        self.syn_node_path(db).path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn submodule_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: SubmoduleSynNodePath,
) -> SubmoduleSynNode {
    let module_path = syn_node_path.module_path(db);
    let item_tree_sheet = db
        .item_syn_tree_sheet(module_path)
        .expect("should be valid");
    match item_tree_sheet.major_item_node(syn_node_path.into()) {
        Some(ItemSynNode::Submodule(node)) => node,
        _ => unreachable!(),
    }
}
