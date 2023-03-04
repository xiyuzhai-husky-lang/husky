use salsa::assert_eq_with_db;

use crate::*;

#[inline(always)]
pub fn raw_term_entity_path_raw_ty(
    db: &dyn RawTypeDb,
    path: RawTermEntityPath,
) -> RawTypeResult<ReducedRawTerm> {
    match path {
        RawTermEntityPath::Form(_) => todo!(),
        RawTermEntityPath::Trait(_) => todo!(),
        RawTermEntityPath::TypeOntology(_) => todo!(),
        RawTermEntityPath::TypeConstructor(_) => todo!(),
    }
}

#[inline(always)]
pub fn entity_path_raw_ty(
    db: &dyn RawTypeDb,
    disambiguation: TypePathDisambiguation,
    path: EntityPath,
) -> RawTypeResult<ReducedRawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => raw_ty_path_raw_ty(db, path, disambiguation),
            ModuleItemPath::Trait(path) => trai_path_raw_ty(db, path),
            ModuleItemPath::Form(path) => form_path_raw_ty(db, path),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

#[test]
fn entity_path_path_raw_term_raw_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let reduced_raw_term_menu = db.reduced_raw_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty = reduced_raw_term_menu.invariant_ty0_to_trai_ty();
    let covariant_lifetime_to_covariant_ty0_to_ty0 =
        reduced_raw_term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0();
    let covariant_lifetime_to_invariant_ty0_to_ty0 =
        reduced_raw_term_menu.covariant_lifetime_to_invariant_ty0_to_ty0();
    let trai_ty = reduced_raw_term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.bool_ty_path().into(),
        ),
        Ok(reduced_raw_term_menu.ty0())
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_add().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_add_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_and().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_and_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_or().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_or_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_xor().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_xor_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_div().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_div_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_mul().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_mul_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_neg().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::RawTypeItselfOrTemplate,
            entity_path_menu.core_ops_not().into(),
        ),
        Ok(trai_ty)
    );
    // assert_eq_with_db!(
    //     db,
    //     entity_path_raw_ty(&db, entity_path_menu.ref_raw_ty_path().into()),
    //     Ok(covariant_lifetime_to_covariant_ty0_to_ty0)
    // );
}

#[salsa::tracked(jar = RawTypeJar)]
pub fn raw_ty_path_raw_ty(
    db: &dyn RawTypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> RawTypeResult<ReducedRawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = raw_ty_entity_variances(db, path) else {
        todo!()
    };
    match disambiguation {
        TypePathDisambiguation::RawTypeItselfOrTemplate => {
            Ok(curry_from_implicit_parameter_raw_tys(
                db,
                CurryKind::Explicit,
                variances,
                signature.implicit_parameters(db),
                raw_term_menu.ty0().into(),
            ))
        }
        TypePathDisambiguation::InstanceOrConstructor => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn trai_path_raw_ty(
    db: &dyn RawTypeDb,
    path: TraitPath,
) -> RawTypeResult<ReducedRawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let Ok(variances) = trai_entity_variances(db, path) else {
        todo!()
    };
    let signature = match db.trai_signature(decl) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    Ok(curry_from_implicit_parameter_raw_tys(
        db,
        CurryKind::Explicit,
        variances,
        signature.implicit_parameters(db),
        raw_term_menu.trai_ty().into(),
    ))
}

#[salsa::tracked(jar = RawTypeJar)]
pub(crate) fn form_path_raw_ty(
    db: &dyn RawTypeDb,
    path: FormPath,
) -> RawTypeResult<ReducedRawTerm> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::DeclError.into()),
    };
    let signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).as_ref().unwrap();
    match signature {
        FormSignature::Function(signature) => {
            function_entity_raw_ty(db, variances, signature, raw_term_menu)
        }
        FormSignature::Feature(signature) => feature_entity_raw_ty(db, signature, raw_term_menu),
        FormSignature::Morphism(_) => todo!(),
        FormSignature::Value(_) => todo!(),
    }
}

pub(crate) fn function_entity_raw_ty(
    db: &dyn RawTypeDb,
    variances: &[Variance],
    signature: FunctionSignature,
    raw_term_menu: &RawTermMenu,
) -> RawTypeResult<ReducedRawTerm> {
    let param_raw_tys = signature
        .parameters(db)
        .iter()
        .map(|param| RawTermRitchieParameter::new(param.ty()))
        .collect();
    let return_raw_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameter_raw_tys(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        RawTermRitchie::new(db, RawTermRitchieKind::Fp, param_raw_tys, return_raw_ty).into(),
    ))
}

pub(crate) fn feature_entity_raw_ty(
    db: &dyn RawTypeDb,
    signature: FeatureSignature,
    raw_term_menu: &RawTermMenu,
) -> RawTypeResult<ReducedRawTerm> {
    Ok(calc_reduced_raw_term(db, signature.return_ty(db)))
}

fn curry_from_implicit_parameter_raw_tys(
    db: &dyn RawTypeDb,
    raw_term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut raw_term: RawTerm,
) -> ReducedRawTerm {
    assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        let symbol = implicit_parameter.symbol();
        assert_eq!(symbol.ty(db), Ok(implicit_parameter.ty()));
        let symbol = db
            .raw_term_contains_symbol(raw_term, symbol)
            .then_some(symbol);
        raw_term = RawTermCurry::new(
            db,
            raw_term_curry_kind,
            *variance,
            symbol,
            implicit_parameter.ty(),
            raw_term,
        )
        .into()
    }
    calc_reduced_raw_term(db, raw_term)
}
