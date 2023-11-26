mod fugitive;
mod trai;
mod ty_constructor;
mod ty_instance_constructor;
mod utils;

pub use self::fugitive::*;

pub use self::ty_instance_constructor::*;

use crate::*;

#[cfg(test)]
use salsa::assert_eq_with_db;
use utils::*;

#[inline(always)]
pub fn declarative_term_item_path_declarative_ty(
    _db: &::salsa::Db,
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
pub fn item_path_declarative_ty(
    db: &::salsa::Db,
    disambiguation: TypePathDisambiguation,
    path: ItemPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match path {
        ItemPath::Submodule(_, _) => Ok(declarative_term_menu.module()),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => match disambiguation {
                TypePathDisambiguation::OntologyConstructor => {
                    ty_ontology_path_declarative_ty(db, path)
                }
                TypePathDisambiguation::InstanceConstructor => {
                    ty_instance_constructor_path_declarative_ty(db, path)
                }
            },
            MajorItemPath::Trait(path) => trai_path_declarative_ty(db, path),
            MajorItemPath::Fugitive(path) => fugitive_path_declarative_ty(db, path),
        },
        ItemPath::AssociatedItem(_) => todo!(),
        ItemPath::TypeVariant(_, _) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_, _) => todo!(),
    }
}

#[test]
fn item_path_declarative_ty_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = item_path_menu(db, toolchain);
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
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bool_ty_path().into(),
        ),
        Ok(declarative_term_menu.ty0().into())
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.add_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.add_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bit_and_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bit_and_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bit_or_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.core_ops_bit_or_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bit_xor_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.bit_xor_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.div_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.div_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.mul_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.mul_assign_trai_path().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.neg_trai_path().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        item_path_declarative_ty(
            db,
            TypePathDisambiguation::OntologyConstructor,
            item_path_menu.not_trai_path().into(),
        ),
        Ok(trai_ty)
    );
    // assert_eq_with_db!(
    //     db,
    //     item_path_declarative_ty(db, item_path_menu.ref_declarative_ty_path().into()),
    //     Ok(covariant_lifetime_to_covariant_ty0_to_ty0)
    // );
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_ontology_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = ty_template_parameter_variances(db, path) else {
        todo!()
    };
    curry_from_template_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.template_parameters(db),
        declarative_term_menu.ty0(),
    )
}

#[test]
fn ty_ontology_path_declarative_ty_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = item_path_menu(db, toolchain);
    let _array_ty_ontology_path_declarative_ty =
        ty_ontology_path_declarative_ty(db, item_path_menu.array_ty_path());
    // use husky_print_utils::*;
    // use salsa::DebugWithDb;
    // p!(array_ty_ontology_path_declarative_ty.debug(&db));
    // todo!()
}

// todo: this should return a template
#[deprecated(note = "it's better to use signature directly instead of invoking this function")]
#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_variant_path_declarative_ty(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    // todo: GADT
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature_template = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    match signature_template {
        TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Unit(signature_template) => {
            let Ok(parent_ty_template_parameter_variances) =
                ty_template_parameter_variances(db, path.parent_ty_path(db))
            else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_template_parameters(
                db,
                CurryKind::Implicit,
                parent_ty_template_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .template_parameters(db),
                signature_template.ty(db),
            )
        }
        TypeVariantDeclarativeSignatureTemplate::Tuple(signature_template) => {
            let Ok(parent_ty_template_parameter_variances) =
                ty_template_parameter_variances(db, path.parent_ty_path(db))
            else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_template_parameters(
                db,
                CurryKind::Implicit,
                parent_ty_template_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .template_parameters(db),
                signature_template.instance_constructor_ty(db),
            )
        }
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn trai_path_declarative_ty(
    db: &::salsa::Db,
    path: TraitPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let Ok(variances) = trai_item_variances(db, path) else {
        todo!()
    };
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    curry_from_template_parameters(
        db,
        CurryKind::Explicit,
        variances,
        signature.template_parameters_without_self_ty(db),
        declarative_term_menu.trai_ty(),
    )
}
