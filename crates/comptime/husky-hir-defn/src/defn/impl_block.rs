use super::*;

impl HasHirNodeDefn for ImplBlockHirNodePath {
    type NodeDefn = ImplBlockHirNodeDecl;

    fn syn_node_defn(self, db: &dyn HirDefnDb) -> Self::NodeDefn {
        self.syn_node_decl(db)
    }
}

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDecl;

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        Ok(self.decl(db)?)
    }
}
