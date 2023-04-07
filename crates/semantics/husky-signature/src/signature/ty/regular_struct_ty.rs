use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> SignatureResult<RegularStructTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(RegularStructTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            signature_term_region,
            raw_term_menu,
        ),
        decl.fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(RegularStructFieldSignature {
                    ident: field.ident(),
                    ty: match signature_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(SignatureError::FieldTypeRawTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
                })
            })
            .collect::<SignatureResult<Vec<_>>>()?,
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct RegularStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar= SignatureJar)]
pub struct RegularStructFieldSignature {
    ident: Ident,
    ty: RawTerm,
}

impl RegularStructFieldSignature {
    pub fn into_ritchie_parameter_contracted_ty(self) -> RawTermRitchieParameterContractedType {
        RawTermRitchieParameterContractedType::new(PatternContract::Move, self.ty)
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> RawTerm {
        self.ty
    }
}
