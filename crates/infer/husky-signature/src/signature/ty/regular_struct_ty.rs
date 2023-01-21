use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeSignature {
    todo!()
    // RegularStructTypeSignature::new(
    //     db,
    //     ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
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
    pub term_sheet: SignatureTermSheet,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldSignature {
    ident: Identifier,
    ty: SignatureTermOutcome<Term>,
}
