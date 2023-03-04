use super::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn regular_struct_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: RegularStructTypeDecl,
) -> RawSignatureResult<RegularStructTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(RegularStructTypeRawSignature::new(
        db,
        ImplicitParameterRawSignatures::from_decl(
            decl.implicit_parameters(db)?,
            raw_signature_term_region,
            raw_term_menu,
        ),
        decl.fields(db)?
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(RegularStructFieldRawSignature {
                    ident: field.ident(),
                    ty: match raw_signature_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(RawSignatureError::FieldTypeRawTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
                })
            })
            .collect::<RawSignatureResult<Vec<_>>>()?,
    ))
}

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct RegularStructTypeRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
    #[return_ref]
    pub fields: Vec<RegularStructFieldRawSignature>,
}

impl RegularStructTypeRawSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldRawSignature {
    ident: Identifier,
    ty: RawTerm,
}
