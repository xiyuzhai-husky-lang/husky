use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AttrSynNodeDefn {
    syn_node_decl: AttrSynNodeDecl,
}

impl AttrSynNodeDefn {
    pub fn syn_node_decl(&self) -> AttrSynNodeDecl {
        self.syn_node_decl
    }
}

impl HasSynNodeDefn for AttrSynNodePath {
    type SynNodeDefn = AttrSynNodeDefn;

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        AttrSynNodeDefn {
            syn_node_decl: self.syn_node_decl(db),
        }
    }
}
