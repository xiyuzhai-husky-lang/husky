use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_instance_constructor_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) =  ty_implicit_parameter_variances(db, path) else {
        todo!()
    };
    match signature {
        TypeDeclarativeSignatureTemplate::Enum(_) => {
            Err(OriginalDeclarativeTypeError::EnumTypeHasNoConstructor)?
        }
        TypeDeclarativeSignatureTemplate::PropsStruct(signature) => {
            Ok(props_struct_ty_instance_constructor_path_declarative_ty(
                db, path, variances, signature,
            )?)
        }
        TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::TupleStruct(signature) => Ok(
            tuple_struct_ty_constructor_path_declarative_ty(db, path, variances, signature)?,
        ),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Inductive(_) => {
            Err(OriginalDeclarativeTypeError::InductiveTypeHasNoConstructor)?
        }
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Extern(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    }
}

fn props_struct_ty_instance_constructor_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    variances: &[Variance],
    signature: PropsStructDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let implicit_parameters = &signature.implicit_parameters(db);
    let self_ty = signature.self_ty(db);
    let parameter_tys = signature
        .fields(db)
        .iter()
        .copied()
        .map(PropsStructFieldDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let instance_constructor_ty =
        DeclarativeTermRitchie::new(db, RitchieKind::FnType, parameter_tys, self_ty);
    curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        implicit_parameters,
        instance_constructor_ty,
    )
}

fn tuple_struct_ty_constructor_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    variances: &[Variance],
    signature: TupleStructDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let implicit_parameters = &signature.implicit_parameters(db);
    let self_ty = signature.self_ty(db);
    let parameter_tys = signature
        .fields(db)
        .iter()
        .copied()
        .map(TupleStructFieldDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let constructor_ty =
        DeclarativeTermRitchie::new(db, RitchieKind::FnType, parameter_tys, self_ty);
    curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        implicit_parameters,
        constructor_ty,
    )
}
