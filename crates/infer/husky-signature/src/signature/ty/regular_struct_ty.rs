use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    todo!()
    // RegularStructTypeSignature::new(
    //     db,
    //     ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region),
    //     decl.fields(db)
    //         .iter()
    //         .map(|field| RegularStructFieldSignature {
    //             ident: field.ident(),
    //             ty: engine.query_new(field.ty()),
    //         })
    //         .collect(),
    // )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct RegularStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
    #[return_ref]
    pub term_sheet: SignatureTermRegion,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldSignature {
    ident: Identifier,
    ty: SignatureTermOutcome<Term>,
}
