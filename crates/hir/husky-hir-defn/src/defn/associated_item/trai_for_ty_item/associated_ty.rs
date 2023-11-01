use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedTypeHirDecl,
}

impl TraitForTypeAssociatedTypeHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeAssociatedTypeHirDecl,
    ) -> Self {
        TraitForTypeAssociatedTypeHirDefn::new_inner(db, path, hir_decl)
    }
}
