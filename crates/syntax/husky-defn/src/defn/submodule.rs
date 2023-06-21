use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DefnDb, jar = DefnJar)]
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
