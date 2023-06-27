use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub(crate) fn ty_item_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypeItemPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let signature = path
        .declarative_signature_template(db)
        .map_err(|_| DerivedDeclarativeTypeError::SignatureError)?;
    let Ok(variances) = ty_item_entity_variances(db, path) else {
        todo!()
    };
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match signature {
        TypeItemDeclarativeSignatureTemplate::AssociatedFn(signature) => {
            ty_associated_fn_path_declarative_ty(db, variances, signature)
        }
        TypeItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
        TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
        TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
        TypeItemDeclarativeSignatureTemplate::MemoizedField(_) => todo!(),
    }
}

fn ty_associated_fn_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: TypeAssociatedFnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let param_declarative_tys = signature
        .parameters(db)
        .iter()
        .copied()
        .map(ExplicitParameterDeclarativeSignature::into_ritchie_parameter_contracted_ty)
        .collect();
    let return_declarative_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            param_declarative_tys,
            return_declarative_ty,
        ),
    ))
}
