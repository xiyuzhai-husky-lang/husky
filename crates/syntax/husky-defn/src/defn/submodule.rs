use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DefnDb, jar = SynDefnJar)]
pub struct SubmoduleNodeDefn {
    node_decl: SubmoduleNodeDecl,
}

impl SubmoduleNodeDefn {
    pub fn node_decl(self) -> SubmoduleNodeDecl {
        self.node_decl
    }
}

impl HasNodeDefn for SubmoduleNodePath {
    type NodeDefn = SubmoduleNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        SubmoduleNodeDefn {
            node_decl: self.node_decl(db),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DefnDb, jar = SynDefnJar)]
pub struct SubmoduleDefn {
    decl: SubmoduleDecl,
}

impl SubmoduleDefn {
    pub fn decl(self) -> SubmoduleDecl {
        self.decl
    }
}

impl HasDefn for ModulePath {
    type Defn = SubmoduleDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        Ok(SubmoduleDefn {
            decl: self.decl(db)?,
        })
    }
}
