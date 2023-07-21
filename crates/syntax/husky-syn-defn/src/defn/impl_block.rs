use super::*;

impl HasSynNodeDefn for ImplBlockSynNodePath {
    type NodeDefn = ImplBlockSynNodeDecl;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        self.node_decl(db)
    }
}

impl HasDefn for ImplBlockPath {
    type Defn = ImplBlockSynDecl;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        Ok(self.decl(db)?)
    }
}
