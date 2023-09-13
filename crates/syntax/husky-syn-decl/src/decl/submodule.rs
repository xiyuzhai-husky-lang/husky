use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct SubmoduleSynNodeDecl {
    #[id]
    pub syn_node_path: SubmoduleSynNodePath,
}

impl SubmoduleSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        Default::default()
    }
}

impl HasSynNodeDecl for SubmoduleSynNodePath {
    type NodeDecl = SubmoduleSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        submodule_syn_node_decl(db, self)
    }
}

#[salsa::tracked( jar = SynDeclJar)]
pub(crate) fn submodule_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: SubmoduleSynNodePath,
) -> SubmoduleSynNodeDecl {
    SubmoduleSynNodeDecl::new(db, syn_node_path)
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct SubmoduleSynDecl {
    #[id]
    pub path: SubmodulePath,
}

impl SubmoduleSynDecl {
    #[inline(always)]
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: SubmodulePath,
        syn_node_decl: SubmoduleSynNodeDecl,
    ) -> Self {
        Self::new(db, path)
    }
}

// actually it only works for nonroot module path
// but rust doesn't provide refinement type
impl HasSynDecl for SubmodulePath {
    type Decl = SubmoduleSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        submodule_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn submodule_decl(
    db: &dyn SynDeclDb,
    path: SubmodulePath,
) -> DeclResult<SubmoduleSynDecl> {
    let syn_node_path = path.syn_node_path(db);
    let syn_node_decl = syn_node_path.syn_node_decl(db);
    Ok(SubmoduleSynDecl::from_node_decl(db, path, syn_node_decl))
}
