use crate::*;

pub(crate) fn entity_path_ty(db: &dyn TypeDb, path: EntityPath) -> TypeResult<ReducedTerm> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_entity_ty(db, path),
            ModuleItemPath::Trait(path) => trai_entity_ty(db, path),
            ModuleItemPath::Form(path) => form_entity_ty(db, path),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

macro_rules! assert_eq_with_db {
    ($db: expr, $left: expr, $right: expr) => {
        if let Err(error_message) = assert_eq_with_db_f(&$db, &$left, &$right) {
            panic!("{}", error_message)
        }
    };
}

fn assert_eq_with_db_f<Db, T>(db: &Db, left: &T, right: &T) -> Result<(), String>
where
    T: PartialEq + salsa::DebugWithDb<Db>,
{
    if left != right {
        use salsa::DebugWithDb;
        Err(format!(
            r#"assertion failed: `(left == right)`
left: `{:?}`,
right: `{:?}`"#,
            left.debug(db),
            right.debug(db)
        ))
    } else {
        Ok(())
    }
}

#[test]
fn entity_path_path_term_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let reduced_term_menu = db.reduced_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty = reduced_term_menu.invariant_ty0_to_trai_ty();
    let trai_ty = reduced_term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.bool().into()),
        Ok(reduced_term_menu.ty0())
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_add().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_add_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_and().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_and_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_or().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_or_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_xor().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_bit_xor_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_div().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_div_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_mul().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_mul_assign().into()),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_neg().into()),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(&db, entity_path_menu.core_ops_not().into()),
        Ok(trai_ty)
    );
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn ty_entity_ty(db: &dyn TypeDb, path: TypePath) -> TypeResult<ReducedTerm> {
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

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn trai_entity_ty(db: &dyn TypeDb, path: TraitPath) -> TypeResult<ReducedTerm> {
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
        term_menu.trai_ty().into(),
    ))
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn form_entity_ty(db: &dyn TypeDb, path: FormPath) -> TypeResult<ReducedTerm> {
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
) -> TypeResult<ReducedTerm> {
    let param_tys = signature
        .parameters(db)
        .iter()
        .map(|param| TermRitchieParameter::new(param.ty()))
        .collect();
    let return_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameter_tys(
        db,
        variances,
        signature.implicit_parameters(db),
        TermRitchie::new(db, TermRitchieKind::Fp, param_tys, return_ty).into(),
    ))
}

pub(crate) fn feature_entity_ty(
    db: &dyn TypeDb,
    signature: FeatureSignature,
    term_menu: &TermMenu,
) -> TypeResult<ReducedTerm> {
    Ok(calc_reduced_term(db, signature.return_ty(db)))
}

fn curry_from_implicit_parameter_tys(
    db: &dyn TypeDb,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> ReducedTerm {
    assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        term = TermCurry::new(db, *variance, implicit_parameter.ty(), term).into()
    }
    calc_reduced_term(db, term)
}
