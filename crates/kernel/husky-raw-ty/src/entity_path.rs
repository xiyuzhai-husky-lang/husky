mod associated_item;
mod form;
mod trai;
mod ty_constructor;
mod ty_ontology;
mod utils;

pub use self::associated_item::*;
pub use self::form::*;
pub use self::trai::*;
pub use self::ty_constructor::*;
pub use self::ty_ontology::*;

use crate::*;
use husky_decl::HasDecl;
#[cfg(test)]
use salsa::assert_eq_with_db;
use utils::*;

#[inline(always)]
pub fn raw_term_entity_path_raw_ty(
    _db: &dyn RawTypeDb,
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
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
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
        EntityPath::TypeVariant(_) => todo!(),
    }
}

#[test]
fn entity_path_raw_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let raw_term_menu = db.raw_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty: RawTerm = raw_term_menu.invariant_ty0_to_trai_ty().into();
    let _ex_co_lifetime_to_ex_co_ty0_to_ty0: RawTerm =
        raw_term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0().into();
    let _ex_co_lifetime_to_ex_inv_ty0_to_ty0: RawTerm =
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
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_e) => {
            return Err(DerivedRawTypeError::TypeOntologyDeclError { path }.into());
        }
    };
    let signature = match db.ty_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedRawTypeError::SignatureError.into()),
    };
    let Ok(variances) =  ty_entity_variances(db, path) else {
        todo!()
    };
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.implicit_parameters(db),
        raw_term_menu.ty0(),
    ))
}

#[salsa::tracked(jar = RawTypeJar)]
pub fn trai_path_raw_ty(db: &dyn RawTypeDb, path: TraitPath) -> RawTypeResult<RawTerm> {
    let raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match path.decl(db) {
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
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.implicit_parameters(db),
        raw_term_menu.trai_ty(),
    ))
}
