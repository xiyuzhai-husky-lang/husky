use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct SubmoduleNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<ModulePath>,
}

impl SubmoduleNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: ModulePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    // gives the parent
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db)
            .path
            .parent(db)
            .expect("non root")
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<ModulePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> SubmoduleNode {
        submodule_node(db, self)
    }
}

impl HasNodePath for ModulePath {
    type NodePath = SubmoduleNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        SubmoduleNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct SubmoduleNode {
    #[id]
    pub node_path: SubmoduleNodePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

impl SubmoduleNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        submodule_path: ModulePath,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> Self {
        Self::new_inner(
            db,
            SubmoduleNodePath::new(db, registry, submodule_path),
            visibility,
            ast_idx,
            ident_token,
        )
    }

    pub fn unambiguous_path(self, db: &dyn EntityTreeDb) -> Option<ModulePath> {
        self.node_path(db).path(db)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn submodule_node(db: &dyn EntityTreeDb, node_path: SubmoduleNodePath) -> SubmoduleNode {
    let module_path = node_path.module_path(db);
    let entity_tree_sheet = db.entity_tree_sheet(module_path).expect("should be valid");
    match entity_tree_sheet.major_entity_node(node_path.into()) {
        Some(EntityNode::Submodule(node)) => node,
        _ => unreachable!(),
    }
}
