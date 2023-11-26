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

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        SubmoduleSynNodeDefn {
            syn_node_decl: self.syn_node_decl(db),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
pub struct SubmoduleSynDefn {
    decl: SubmoduleSynDecl,
}

impl SubmoduleSynDefn {
    pub fn decl(self) -> SubmoduleSynDecl {
        self.decl
    }

    pub fn path(self, db: &::salsa::Db) -> SubmodulePath {
        self.decl.path(db)
    }
}

impl HasSynDefn for SubmodulePath {
    type SynDefn = SubmoduleSynDefn;

    fn syn_defn(self, db: &::salsa::Db) -> SynDefnResult<Self::SynDefn> {
        Ok(SubmoduleSynDefn {
            decl: self.syn_decl(db)?,
        })
    }
}
