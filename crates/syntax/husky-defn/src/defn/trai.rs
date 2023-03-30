use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub path: TraitPath,
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
    let path = decl.path(db);
    TraitDefn::new(db, path, decl)
}
