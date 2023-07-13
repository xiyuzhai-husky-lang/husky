mod fugitive;
mod trai;
mod ty_constructor;
mod ty_instance_constructor;
mod utils;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty_constructor::*;
pub use self::ty_instance_constructor::*;

use crate::*;
use husky_decl::{HasDecl, TypeVariantDecl};
#[cfg(test)]
use salsa::assert_eq_with_db;
use utils::*;

#[inline(always)]
pub fn declarative_term_entity_path_declarative_ty(
    _db: &dyn DeclarativeTypeDb,
    path: DeclarativeTermEntityPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    match path {
        DeclarativeTermEntityPath::Fugitive(_) => todo!(),
        DeclarativeTermEntityPath::Trait(_) => todo!(),
        DeclarativeTermEntityPath::Type(_) => todo!(),
        DeclarativeTermEntityPath::TypeVariant(_) => todo!(),
    }
}

#[inline(always)]
pub fn entity_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    disambiguation: TypePathDisambiguation,
    path: EntityPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match path {
        EntityPath::Module(_) => Ok(declarative_term_menu.module()),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => match disambiguation {
                TypePathDisambiguation::OntologyConstructor => {
                    ty_ontology_path_declarative_ty(db, path)
                }
                TypePathDisambiguation::InstanceConstructor => {
                    ty_instance_constructor_path_declarative_ty(db, path)
                }
            },
            ModuleItemPath::Trait(path) => trai_path_declarative_ty(db, path),
            ModuleItemPath::Fugitive(path) => form_path_declarative_ty(db, path),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::TypeVariant(_) => todo!(),
        EntityPath::ImplBlock(_) => todo!(),
    }
}

#[test]
fn entity_path_declarative_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let declarative_term_menu = db.declarative_term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty: DeclarativeTerm =
        declarative_term_menu.invariant_ty0_to_trai_ty().into();
    let _ex_co_lifetime_to_ex_co_ty0_to_ty0: DeclarativeTerm = declarative_term_menu
        .ex_co_lifetime_to_ex_co_ty0_to_ty0()
        .into();
    let _ex_co_lifetime_to_ex_inv_ty0_to_ty0: DeclarativeTerm = declarative_term_menu
        .ex_co_lifetime_to_ex_inv_ty0_to_ty0()
        .into();
    let trai_ty = declarative_term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.bool_ty_path().into(),
        ),
        Ok(declarative_term_menu.ty0().into())
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_add_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_add_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_and_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_and_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_or_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_or_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_xor_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_bit_xor_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_div_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_div_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_mul_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_mul_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_neg_trai_path().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_declarative_ty(
            &db,
            TypePathDisambiguation::OntologyConstructor,
            entity_path_menu.core_ops_not_trai_path().into(),
        ),
        Ok(trai_ty)
    );
    // assert_eq_with_db!(
    //     db,
    //     entity_path_declarative_ty(&db, entity_path_menu.ref_declarative_ty_path().into()),
    //     Ok(covariant_lifetime_to_covariant_ty0_to_ty0)
    // );
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_ontology_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) =  ty_implicit_parameter_variances(db, path) else {
        todo!()
    };
    curry_from_generic_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.generic_parameters(db),
        declarative_term_menu.ty0(),
    )
}

#[test]
fn ty_ontology_path_declarative_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let array_ty_ontology_path_declarative_ty =
        ty_ontology_path_declarative_ty(&db, entity_path_menu.array_ty_path());
    // use husky_print_utils::*;
    // use salsa::DebugWithDb;
    // p!(array_ty_ontology_path_declarative_ty.debug(&db));
    // todo!()
}

// todo: this should return a template
#[deprecated(note = "it's better to use signature directly instead of invoking this function")]
#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_variant_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TypeVariantPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    // todo: GADT
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature_template = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    match signature_template {
        TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Unit(signature_template) => {
            let Ok(parent_ty_implicit_parameter_variances) =  ty_implicit_parameter_variances(db, path.parent_ty_path(db)) else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_generic_parameters(
                db,
                CurryKind::Implicit,
                parent_ty_implicit_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .generic_parameters(db),
                signature_template.ty(db),
            )
        }
        TypeVariantDeclarativeSignatureTemplate::Tuple(signature_template) => {
            let Ok(parent_ty_implicit_parameter_variances) =  ty_implicit_parameter_variances(db, path.parent_ty_path(db)) else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_generic_parameters(
                db,
                CurryKind::Implicit,
                parent_ty_implicit_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .generic_parameters(db),
                signature_template.instance_constructor_ty(db),
            )
        }
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn trai_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: TraitPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let Ok(variances) = trai_entity_variances(db, path) else {
        todo!()
    };
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    curry_from_generic_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.generic_parameters(db),
        declarative_term_menu.trai_ty(),
    )
}
