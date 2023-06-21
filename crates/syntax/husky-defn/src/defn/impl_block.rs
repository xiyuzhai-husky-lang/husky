use super::*;

impl HasDefn for ImplBlockPath {
    type Defn = ImplBlockDecl;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        Ok(self.decl(db)?)
    }
}
