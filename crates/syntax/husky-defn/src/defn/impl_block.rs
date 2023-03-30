use super::*;

impl HasDefn for ImplBlockDecl {
    type Defn = ImplBlockDecl;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        self
    }
}
