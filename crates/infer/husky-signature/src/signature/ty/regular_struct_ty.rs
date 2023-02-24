use super::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> SignatureResult<RegularStructTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(RegularStructTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db)?,
            signature_term_region,
            term_menu,
        ),
        decl.fields(db)?
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(RegularStructFieldSignature {
                    ident: field.ident(),
                    ty: match signature_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(SignatureError::FieldTypeTermError(i.try_into().unwrap()))
                        }
                    },
                })
            })
            .collect::<SignatureResult<Vec<_>>>()?,
    ))
}

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct RegularStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldSignature {
    ident: Identifier,
    ty: Term,
}
