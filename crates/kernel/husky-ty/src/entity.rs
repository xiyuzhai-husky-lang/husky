use crate::*;

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn ty_entity_ty(db: &dyn TypeDb, path: TypePath) -> TypeResult<Term> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedTypeError::SignatureError.into()),
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        signature.implicit_parameters(db),
        term_menu.ty0(),
    ))
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn trai_entity_ty(db: &dyn TypeDb, path: TraitPath) -> TypeResult<Term> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let signature = match db.trai_signature(decl) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        signature.implicit_parameters(db),
        term_menu.ty0(),
    ))
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn form_entity_ty(db: &dyn TypeDb, path: FormPath) -> TypeResult<Term> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let signature = db.form_signature(decl);
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match signature {
        Ok(signature) => match signature {
            FormSignature::Function(signature) => function_entity_ty(db, signature, term_menu),
            FormSignature::Feature(signature) => feature_entity_ty(db, signature, term_menu),
            FormSignature::Morphism(_) => todo!(),
            FormSignature::Value(_) => todo!(),
        },
        Err(_) => Err(DerivedTypeError::SignatureError.into()),
    }
}

pub(crate) fn function_entity_ty(
    db: &dyn TypeDb,
    signature: FunctionSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    let param_tys = signature
        .parameters(db)
        .iter()
        .map(|param| TermDurantParameter::new(param.ty()))
        .collect();
    let output_ty = signature.output_ty(db);
    Ok(Term::Durant(TermDurant::new(
        db,
        TermDurantKind::Fp,
        param_tys,
        output_ty,
    )))
}

pub(crate) fn feature_entity_ty(
    db: &dyn TypeDb,
    signature: FeatureSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    Ok(signature.output_ty(db))
}

fn curry_from_implicit_parameter_tys(
    db: &dyn TypeDb,
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> Term {
    for implicit_parameter in implicit_parameters.iter().rev() {
        term = TermCurry::new(db, implicit_parameter.ty(), term).into()
    }
    term
}
