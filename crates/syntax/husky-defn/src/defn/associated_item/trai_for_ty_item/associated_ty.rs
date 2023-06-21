use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node_decl: TraitForTypeAssociatedTypeNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedTypeDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

// impl HasDefn for TraitForTypeAssociatedTypeDecl {
//     type Defn = TraitForTypeAssociatedTypeDefn;

//     fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
//         trai_for_ty_associated_ty_defn(db, self)
//     }
// }

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_ty_defn(
    db: &dyn DefnDb,
    decl: TraitForTypeAssociatedTypeDecl,
) -> TraitForTypeAssociatedTypeDefn {
    todo!()
    // let node_path = decl.node_path(db);
    // let mut parser = expr_parser(
    //     db,
    //     decl.node_path(db),
    //     Some(decl.expr_region(db)),
    //     AllowSelfType::True,
    //     AllowSelfValue::True,
    // );
    // TraitForTypeAssociatedTypeDefn::new(db, node_path, decl, parser.finish())
}
