use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
pub struct SubmoduleSynNodeDefn {
    syn_node_decl: SubmoduleSynNodeDecl,
}

impl SubmoduleSynNodeDefn {
    pub fn syn_node_decl(self) -> SubmoduleSynNodeDecl {
        self.syn_node_decl
    }
}

impl HasSynNodeDefn for SubmoduleSynNodePath {
    type SynNodeDefn = SubmoduleSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        SubmoduleSynNodeDefn {
            syn_node_decl: self.syn_node_decl(db),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
pub struct SubmoduleDefn {
    decl: SubmoduleSynDecl,
}

impl SubmoduleDefn {
    pub fn decl(self) -> SubmoduleSynDecl {
        self.decl
    }
}

impl HasSynDefn for SubmodulePath {
    type SynDefn = SubmoduleDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(SubmoduleDefn {
            decl: self.syn_decl(db)?,
        })
    }
}
