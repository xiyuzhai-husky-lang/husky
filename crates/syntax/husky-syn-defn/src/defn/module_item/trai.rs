use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitSynNodeDefn {
    #[id]
    pub syn_node_path: TraitSynNodePath,
    pub syn_node_decl: TraitSynNodeDecl,
}

impl HasSynNodeDefn for TraitSynNodePath {
    type NodeDefn = TraitSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        trai_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_syn_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: TraitSynNodePath,
) -> TraitSynNodeDefn {
    let syn_node_decl = syn_node_path.syn_node_decl(db);
    TraitSynNodeDefn::new(db, syn_node_path, syn_node_decl)
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitSynDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitSynDecl,
}

impl HasSynDefn for TraitPath {
    type SynDefn = TraitSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        trai_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn trai_syn_defn(db: &dyn SynDefnDb, path: TraitPath) -> SynDefnResult<TraitSynDefn> {
    let decl = path.syn_decl(db)?;
    Ok(TraitSynDefn::new(db, path, decl))
}
