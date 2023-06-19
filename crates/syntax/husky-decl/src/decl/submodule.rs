use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct SubmoduleNodeDecl {
    #[id]
    pub node_id: SubmoduleNodeId,
    pub ast_idx: AstIdx,
}

impl HasNodeDecl for SubmoduleNodeId {
    type NodeDecl = SubmoduleNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        submodule_node_decl(db, self)
    }
}

#[salsa::tracked( jar = DeclJar)]
pub(crate) fn submodule_node_decl(db: &dyn DeclDb, node_id: SubmoduleNodeId) -> SubmoduleNodeDecl {
    let node = node_id.node(db);
    SubmoduleNodeDecl::new(db, node_id, node.ast_idx(db))
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct SubmoduleDecl {
    #[id]
    pub path: ModulePath,
    pub ast_idx: AstIdx,
}

impl SubmoduleDecl {
    #[inline(always)]
    fn from_node_decl(db: &dyn DeclDb, path: ModulePath, node_decl: SubmoduleNodeDecl) -> Self {
        Self::new(db, path, node_decl.ast_idx(db))
    }
}

// actually it only works for nonroot module path
// but rust doesn't provide refinement type
impl HasDecl for ModulePath {
    type Decl = SubmoduleDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        submodule_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn submodule_decl(db: &dyn DeclDb, path: ModulePath) -> DeclResult<SubmoduleDecl> {
    let node_id = path.node_id(db);
    let node_decl = node_id.node_decl(db);
    Ok(SubmoduleDecl::from_node_decl(db, path, node_decl))
}
