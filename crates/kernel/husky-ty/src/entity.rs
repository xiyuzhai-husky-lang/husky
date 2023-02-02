use crate::*;

pub(crate) fn entity_ty(db: &dyn TypeDb, entipath: EntityPath) -> Result<Term, &TypeError> {
    let term_menu = db.term_menu(entipath.toolchain(db)).as_ref().unwrap();
    match entipath {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_entity_ty(db, path).as_ref().copied(),
            ModuleItemPath::Trait(path) => trai_entity_ty(db, path).as_ref().copied(),
            ModuleItemPath::Form(path) => form_entity_ty(db, path).as_ref().copied(),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

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
    let Ok(variances) = ty_entity_variances(db, path) else {
        todo!()
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        variances,
        signature.implicit_parameters(db),
        term_menu.ty0().into(),
    ))
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn trai_entity_ty(db: &dyn TypeDb, path: TraitPath) -> TypeResult<Term> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let Ok(variances) = trai_entity_variances(db, path) else {
        todo!()
    };
    let signature = match db.trai_signature(decl) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        variances,
        signature.implicit_parameters(db),
        term_menu.ty0().into(),
    ))
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn form_entity_ty(db: &dyn TypeDb, path: FormPath) -> TypeResult<Term> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedTypeError::DeclError.into()),
    };
    let signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match signature {
        FormSignature::Function(signature) => {
            function_entity_ty(db, variances, signature, term_menu)
        }
        FormSignature::Feature(signature) => feature_entity_ty(db, signature, term_menu),
        FormSignature::Morphism(_) => todo!(),
        FormSignature::Value(_) => todo!(),
    }
}

pub(crate) fn function_entity_ty(
    db: &dyn TypeDb,
    variances: &[Variance],
    signature: FunctionSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    let param_tys = signature
        .parameters(db)
        .iter()
        .map(|param| TermDurantParameter::new(param.ty()))
        .collect();
    let output_ty = signature.output_ty(db);
    Ok(curry_from_implicit_parameter_tys(
        db,
        variances,
        signature.implicit_parameters(db),
        TermDurant::new(db, TermDurantKind::Fp, param_tys, output_ty).into(),
    ))
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
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> Term {
    assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        term = TermCurry::new(db, *variance, implicit_parameter.ty(), term).into()
    }
    term
}
