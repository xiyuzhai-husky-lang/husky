use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitDecl,
}

impl HasDefn for TraitPath {
    type Defn = TraitDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        trai_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_defn(db: &dyn DefnDb, path: TraitPath) -> DefnResult<TraitDefn> {
    todo!()
    // let node_path = decl.node_path(db);
    // TraitDefn::new(db, node_path, decl)
}
