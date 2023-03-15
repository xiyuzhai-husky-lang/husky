use super::*;

#[salsa::tracked(jar = RawTypeJar)]
pub fn ty_constructor_path_raw_ty(db: &dyn RawTypeDb, path: TypePath) -> RawTypeResult<RawTerm> {
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypeConstructorDeclError.into()),
    };
    let signature = match db.ty_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = raw_ty_entity_variances(db, path) else {
        todo!()
    };
    match signature {
        TypeSignature::Enum(_) => Err(todo!()),
        TypeSignature::RegularStruct(signature) => Ok(regular_struct_ty_constructor_path_raw_ty(
            db, path, variances, signature,
        )),
        TypeSignature::UnitStruct(_) => todo!(),
        TypeSignature::TupleStruct(_) => todo!(),
        TypeSignature::Record(_) => todo!(),
        TypeSignature::Inductive(_) => {
            Err(OriginalRawTypeError::InductiveTypeHasNoConstructor.into())
        }
        TypeSignature::Structure(_) => todo!(),
        TypeSignature::Foreign(_) => todo!(),
        TypeSignature::Union(_) => todo!(),
    }
}

fn regular_struct_ty_constructor_path_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    variances: &[Variance],
    signature: RegularStructTypeSignature,
) -> RawTerm {
    let implicit_parameters = &signature.implicit_parameters(db);
    let self_ty = construct_self_ty(db, path, implicit_parameters);
    let parameter_tys = signature
        .fields(db)
        .map(|field| RawTermRitchieParameter::new(field.ty()));
    let constructor_ty = RawTermRitchie::new(db, TermRitchieKind::Fp, parameter_tys, self_ty);
    curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        implicit_parameters,
        constructor_ty,
    )
}

fn construct_self_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    implicit_parameters: &[ImplicitParameterSignature],
) -> RawTerm {
    let mut self_ty: RawTerm = path.into();
    for implicit_parameter in implicit_parameters {
        self_ty =
            RawTermExplicitApplication::new(db, self_ty, implicit_parameter.symbol().into()).into()
    }
    self_ty
}
