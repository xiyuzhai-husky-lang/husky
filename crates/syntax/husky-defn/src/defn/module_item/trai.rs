use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub node_id: TraitNodeId,
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
    let node_id = decl.node_id(db);
    TraitDefn::new(db, node_id, decl)
}
