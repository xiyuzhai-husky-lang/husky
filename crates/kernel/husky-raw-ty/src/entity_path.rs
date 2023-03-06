use husky_print_utils::p;
use salsa::assert_eq_with_db;

use crate::*;

#[inline(always)]
pub fn raw_term_entity_path_raw_ty(
    db: &dyn RawTypeDb,
    path: RawTermEntityPath,
) -> RawTypeResult<RawTerm> {
    match path {
        RawTermEntityPath::Form(_) => todo!(),
        RawTermEntityPath::Trait(_) => todo!(),
        RawTermEntityPath::Type(_) => todo!(),
    }
}

#[inline(always)]
pub fn entity_path_raw_ty(
    db: &dyn RawTypeDb,
    disambiguation: TypePathDisambiguation,
    path: EntityPath,
) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => match disambiguation {
                TypePathDisambiguation::Ontology => ty_ontology_path_raw_ty(db, path),
                TypePathDisambiguation::Constructor => ty_constructor_path_raw_ty(db, path),
            },
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
    let raw_term_menu = db.raw_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty: RawTerm = raw_term_menu.invariant_ty0_to_trai_ty().into();
    let ex_co_lifetime_to_ex_co_ty0_to_ty0: RawTerm =
        raw_term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0().into();
    let ex_co_lifetime_to_ex_inv_ty0_to_ty0: RawTerm =
        raw_term_menu.ex_co_lifetime_to_ex_inv_ty0_to_ty0().into();
    let trai_ty = raw_term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.bool_ty_path().into(),
        ),
        Ok(raw_term_menu.ty0().into())
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_add().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_add_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_and().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_and_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_or().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_or_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_xor().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_xor_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_div().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_div_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_mul().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_mul_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_neg().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_raw_ty(
            &db,
            TypePathDisambiguation::Ontology,
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
pub fn ty_ontology_path_raw_ty(db: &dyn RawTypeDb, path: TypePath) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(e) => {
            use salsa::DebugWithDb;
            p!(e.debug(db), path.debug(db));
            return Err(DerivedRawTypeError::TypeOntologyDeclError { path }.into());
        }
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = raw_ty_entity_variances(db, path) else {
        todo!()
    };
    Ok(curry_from_implicit_parameter_raw_tys(
        db,
        CurryKind::Explicit,
        variances,
        signature.implicit_parameters(db),
        raw_term_menu.ty0().into(),
    ))
}

#[salsa::tracked(jar = RawTypeJar)]
pub fn ty_constructor_path_raw_ty(db: &dyn RawTypeDb, path: TypePath) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TypeConstructorDeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = raw_ty_entity_variances(db, path) else {
        todo!()
    };
    todo!()
}

#[salsa::tracked(jar = RawTypeJar)]
pub fn trai_path_raw_ty(db: &dyn RawTypeDb, path: TraitPath) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::TraitDeclError.into()),
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
pub fn form_path_raw_ty(db: &dyn RawTypeDb, path: FormPath) -> RawTypeResult<RawTerm> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedRawTypeError::FormDeclError.into()),
    };
    let signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
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
) -> RawTypeResult<RawTerm> {
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
        RawTermRitchie::new(db, TermRitchieKind::Fp, param_raw_tys, return_raw_ty).into(),
    ))
}

pub(crate) fn feature_entity_raw_ty(
    db: &dyn RawTypeDb,
    signature: FeatureSignature,
    raw_term_menu: &RawTermMenu,
) -> RawTypeResult<RawTerm> {
    Ok(signature.return_ty(db))
}

fn curry_from_implicit_parameter_raw_tys(
    db: &dyn RawTypeDb,
    raw_term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut raw_term: RawTerm,
) -> RawTerm {
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
    raw_term
}
