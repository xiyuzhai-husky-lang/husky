use husky_valid_ty::ty_path_valid_ty;
use salsa::assert_eq_with_db;

use crate::*;

#[inline(always)]
pub fn term_entity_path_ty(db: &dyn TypeDb, path: TermEntityPath) -> TypeResult<Term> {
    match path {
        TermEntityPath::Form(_) => todo!(),
        TermEntityPath::Trait(_) => todo!(),
        TermEntityPath::TypeOntology(_) => todo!(),
        TermEntityPath::TypeConstructor(_) => todo!(),
    }
}

#[inline(always)]
pub fn entity_path_ty(
    db: &dyn TypeDb,
    disambiguation: TypePathDisambiguation,
    path: EntityPath,
) -> TypeResult<Term> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_path_ty(db, path, disambiguation),
            ModuleItemPath::Trait(path) => trai_path_ty(db, path),
            ModuleItemPath::Form(path) => form_path_ty(db, path),
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
    let term_menu = db.term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty: Term = term_menu.ex_inv_ty0_to_trai_ty().into();
    let ex_co_lifetime_to_ex_co_ty0_to_ty0: Term =
        term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0().into();
    let ex_co_lifetime_to_ex_ct_ty0_to_ty0: Term =
        term_menu.ex_co_lifetime_to_ex_ct_ty0_to_ty0().into();
    let trai_ty = term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.bool_ty_path().into(),
        ),
        Ok(term_menu.ty0().into())
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_add().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_add_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_and().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_and_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_or().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_or_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_xor().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_bit_xor_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_div().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_div_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_mul().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_mul_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
            entity_path_menu.core_ops_neg().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::TypeItselfOrTemplate,
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
pub fn ty_path_ty(
    db: &dyn TypeDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> TypeResult<Term> {
    Ok(Term::from_valid(
        db,
        ty_path_valid_ty(db, path, disambiguation)?,
    ))
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn trai_path_ty(db: &dyn TypeDb, path: TraitPath) -> TypeResult<Term> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn form_path_ty(db: &dyn TypeDb, path: FormPath) -> TypeResult<Term> {
    todo!()
}

pub(crate) fn function_entity_ty(
    db: &dyn TypeDb,
    variances: &[Variance],
    signature: FunctionSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    todo!()
}

pub(crate) fn feature_entity_ty(
    db: &dyn TypeDb,
    signature: FeatureSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    todo!()
}

fn curry_from_implicit_parameter_tys(
    db: &dyn TypeDb,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> Term {
    todo!()
}
