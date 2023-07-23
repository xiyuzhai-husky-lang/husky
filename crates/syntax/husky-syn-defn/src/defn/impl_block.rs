use super::*;

impl HasSynNodeDefn for ImplBlockSynNodePath {
    type SynNodeDefn = ImplBlockSynNodeDecl;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        self.syn_node_decl(db)
    }
}

impl HasSynDefn for ImplBlockPath {
    type SynDefn = ImplBlockSynDecl;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(self.syn_decl(db)?)
    }
}
