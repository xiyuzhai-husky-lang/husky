use super::*;

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn ty_item_path_raw_ty(
    db: &dyn RawTypeDb,
    path: TypeItemPath,
) -> RawTypeResult<RawTerm> {
    let decl = path.decl(db)?;
    let signature = decl
        .signature(db)
        .map_err(|_| DerivedRawTypeError::SignatureError)?;
    let Ok(variances) = ty_item_entity_variances(db, path) else {
        todo!()
    };
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    match signature {
        TypeItemSignature::AssociatedFn(signature) => {
            ty_associated_fn_path_raw_ty(db, variances, signature)
        }
        TypeItemSignature::MethodFn(_) => todo!(),
        TypeItemSignature::AssociatedType(_) => todo!(),
        TypeItemSignature::AssociatedValue(_) => todo!(),
        TypeItemSignature::Memo(_) => todo!(),
    }
}

fn ty_associated_fn_path_raw_ty(
    db: &dyn RawTypeDb,
    variances: &[Variance],
    signature: TypeAssociatedFnSignature,
) -> RawTypeResult<RawTerm> {
    let param_raw_tys = signature
        .parameters(db)
        .iter()
        .map(|param| RawTermRitchieParameterLiasonedType::new(param.ty()))
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
