use salsa::assert_eq_with_db;

use crate::*;

#[inline(always)]
pub fn entity_path_ty(
    db: &dyn TypeDb,
    context: EntityPathTypeExpectation,
    path: EntityPath,
) -> TypeResult<ReducedTerm> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_path_ty(db, path),
            ModuleItemPath::Trait(path) => trai_entity_ty(db, path),
            ModuleItemPath::Form(path) => form_entity_ty(db, path),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

#[test]
fn entity_path_path_term_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let reduced_term_menu = db.reduced_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty = reduced_term_menu.invariant_ty0_to_trai_ty();
    let covariant_lifetime_to_covariant_ty0_to_ty0 =
        reduced_term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0();
    let covariant_lifetime_to_invariant_ty0_to_ty0 =
        reduced_term_menu.covariant_lifetime_to_invariant_ty0_to_ty0();
    let trai_ty = reduced_term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.bool().into(),
        ),
        Ok(reduced_term_menu.ty0())
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_add().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_add_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_and().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_and_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_or().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_or_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_xor().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_bit_xor_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_div().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_div_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_mul().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_mul_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_neg().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            EntityPathTypeExpectation::Any,
            entity_path_menu.core_ops_not().into(),
        ),
        Ok(trai_ty)
    );
    // assert_eq_with_db!(
    //     db,
    //     entity_path_ty(&db, entity_path_menu.ref_ty_path().into()),
    //     Ok(covariant_lifetime_to_covariant_ty0_to_ty0)
    // );
}

#[salsa::tracked(jar = TypeJar)]
pub fn ty_path_ty(db: &dyn TypeDb, path: TypePath) -> TypeResult<ReducedTerm> {
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
        TermCurryKind::Explicit,
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
        TermCurryKind::Explicit,
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
        TermCurryKind::Implicit,
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
    term_curry_kind: TermCurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> ReducedTerm {
    assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        let symbol = implicit_parameter.symbol();
        assert_eq!(symbol.ty(db), Ok(implicit_parameter.ty()));
        let symbol = db.term_contains_symbol(term, symbol).then_some(symbol);
        term = TermCurry::new(
            db,
            term_curry_kind,
            *variance,
            symbol,
            implicit_parameter.ty(),
            term,
        )
        .into()
    }
    calc_reduced_term(db, term)
}
