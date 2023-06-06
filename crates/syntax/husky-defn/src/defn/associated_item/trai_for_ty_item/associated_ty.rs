use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedTypeDefn {
    pub decl: TraitForTypeAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

impl HasDefn for TraitForTypeAssociatedTypeDecl {
    type Defn = TraitForTypeAssociatedTypeDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        trai_for_ty_associated_ty_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_ty_defn(
    db: &dyn DefnDb,
    decl: TraitForTypeAssociatedTypeDecl,
) -> TraitForTypeAssociatedTypeDefn {
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let expr_region = parser.finish();
    TraitForTypeAssociatedTypeDefn::new(db, decl, expr_region)
}
