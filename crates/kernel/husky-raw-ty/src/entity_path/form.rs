use super::*;

#[salsa::tracked(jar = RawTypeJar)]
pub fn form_path_raw_ty(db: &dyn RawTypeDb, path: FormPath) -> RawTypeResult<RawTerm> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::FormDeclError.into()),
    };
    let signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FormSignature::Function(signature) => {
            function_entity_raw_ty(db, variances, signature, raw_term_menu)
        }
        FormSignature::Feature(signature) => feature_entity_raw_ty(db, signature, raw_term_menu),
        FormSignature::Morphism(_) => todo!(),
        FormSignature::Value(_) => todo!(),
    }
}

pub(crate) fn function_entity_raw_ty(
    db: &dyn RawTypeDb,
    variances: &[Variance],
    signature: FormFnSignature,
    _raw_term_menu: &RawTermMenu,
) -> RawTypeResult<RawTerm> {
    let param_raw_tys = signature
        .parameters(db)
        .iter()
        .map(|param| RawTermRitchieParameter::new(param.ty()))
        .collect();
    let return_raw_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        RawTermRitchie::new(db, TermRitchieKind::Fp, param_raw_tys, return_raw_ty),
    ))
}

pub(crate) fn feature_entity_raw_ty(
    db: &dyn RawTypeDb,
    signature: FeatureSignature,
    _raw_term_menu: &RawTermMenu,
) -> RawTypeResult<RawTerm> {
    Ok(signature.return_ty(db))
}
