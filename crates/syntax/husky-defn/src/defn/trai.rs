use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub node_path: TraitNodePath,
    pub decl: TraitDecl,
}

impl HasDefn for TraitDecl {
    type Defn = TraitDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        trai_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_defn(db: &dyn DefnDb, decl: TraitDecl) -> TraitDefn {
    let node_path = decl.node_path(db);
    TraitDefn::new(db, node_path, decl)
}
