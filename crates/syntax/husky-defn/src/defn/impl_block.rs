use super::*;

impl HasNodeDefn for ImplBlockNodePath {
    type NodeDefn = ImplBlockNodeDecl;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        todo!()
    }
}

impl HasDefn for ImplBlockPath {
    type Defn = ImplBlockDecl;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        Ok(self.decl(db)?)
    }
}
