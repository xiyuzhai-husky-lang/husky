use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DecrSynNodeDefn {
    syn_node_decl: DecrSynNodeDecl,
}

impl DecrSynNodeDefn {
    pub fn syn_node_decl(&self) -> DecrSynNodeDecl {
        self.syn_node_decl
    }
}

impl HasSynNodeDefn for DecrSynNodePath {
    type SynNodeDefn = DecrSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        DecrSynNodeDefn {
            syn_node_decl: self.syn_node_decl(db),
        }
    }
}
