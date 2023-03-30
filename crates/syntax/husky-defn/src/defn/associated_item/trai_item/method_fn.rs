use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitMethodDefn {
    #[id]
    pub entity_path: EntityPath,
    pub decl: TraitMethodDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_method_defn(_db: &dyn DefnDb, _decl: TraitMethodDecl) -> TraitMethodDefn {
    todo!()
}
