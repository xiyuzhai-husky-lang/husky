use super::*;

#[salsa::tracked(jar = RawTypeJar)]
pub fn form_path_raw_ty(db: &dyn RawTypeDb, path: FormPath) -> RawTypeResult<RawTerm> {
    let signature = match path.signature(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FormSignature::Fn(signature) => form_fn_entity_raw_ty(db, variances, signature),
        FormSignature::Feature(signature) => feature_entity_raw_ty(db, signature, raw_term_menu),
        FormSignature::Gn(_) => todo!(),
        FormSignature::Value(_) => todo!(),
    }
}

pub(crate) fn form_fn_entity_raw_ty(
    db: &dyn RawTypeDb,
    variances: &[Variance],
    signature: FnSignature,
) -> RawTypeResult<RawTerm> {
    let param_raw_tys = signature
        .parameters(db)
        .iter()
        .copied()
        .map(ExplicitParameterSignature::into_ritchie_parameter_liasoned_ty)
        .collect();
    let return_raw_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        RawTermRitchie::new(db, TermRitchieKind::FnType, param_raw_tys, return_raw_ty),
    ))
}

pub(crate) fn feature_entity_raw_ty(
    db: &dyn RawTypeDb,
    signature: FeatureSignature,
    _raw_term_menu: &RawTermMenu,
) -> RawTypeResult<RawTerm> {
    Ok(signature.return_ty(db))
}
