use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_constructor_path_raw_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedDeclarativeTypeError::TypeConstructorDeclError.into()),
    };
    let signature = match db.ty_declarative_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) =  ty_entity_variances(db, path) else {
        todo!()
    };
    match signature {
        TypeDeclarativeSignature::Enum(_) => Err(todo!()),
        TypeDeclarativeSignature::RegularStruct(signature) => Ok(
            regular_struct_ty_constructor_path_raw_ty(db, path, variances, signature),
        ),
        TypeDeclarativeSignature::UnitStruct(_) => todo!(),
        TypeDeclarativeSignature::TupleStruct(_) => todo!(),
        TypeDeclarativeSignature::Record(_) => todo!(),
        TypeDeclarativeSignature::Inductive(_) => {
            Err(OriginalDeclarativeTypeError::InductiveTypeHasNoConstructor.into())
        }
        TypeDeclarativeSignature::Structure(_) => todo!(),
        TypeDeclarativeSignature::Foreign(_) => todo!(),
        TypeDeclarativeSignature::Union(_) => todo!(),
    }
}

fn regular_struct_ty_constructor_path_raw_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    variances: &[Variance],
    signature: RegularStructTypeDeclarativeSignature,
) -> DeclarativeTerm {
    let implicit_parameters = &signature.implicit_parameters(db);
    let self_ty = construct_self_ty(db, path, implicit_parameters);
    let parameter_tys = signature
        .fields(db)
        .iter()
        .copied()
        .map(RegularStructFieldSignature::into_ritchie_parameter_contracted_ty)
        .collect();
    let constructor_ty =
        DeclarativeTermRitchie::new(db, TermRitchieKind::FnType, parameter_tys, self_ty);
    curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        implicit_parameters,
        constructor_ty,
    )
}

fn construct_self_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
    implicit_parameters: &[ImplicitParameterSignature],
) -> DeclarativeTerm {
    let mut self_ty: DeclarativeTerm = path.into();
    for implicit_parameter in implicit_parameters {
        self_ty =
            DeclarativeTermExplicitApplication::new(db, self_ty, implicit_parameter.symbol().into())
                .into()
    }
    self_ty
}
