use super::utils;
use crate::*;
use entity_kind::TyKind;
use husky_atom::*;
use husky_entity_route::EntityKind;
use word::RootIdentifier;

#[test]
fn std_scope() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "std",
        AtomVariant::EntityRoute {
            route: RootIdentifier::Std.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn core_scope() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "core",
        AtomVariant::EntityRoute {
            route: RootIdentifier::Core.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn debug_scope() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "debug",
        AtomVariant::EntityRoute {
            route: RootIdentifier::Debug.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn i32_type() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "i32",
        AtomVariant::EntityRoute {
            route: RootIdentifier::I32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn f32_type() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "f32",
        AtomVariant::EntityRoute {
            route: RootIdentifier::F32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn vec_generics() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "Vec",
        AtomVariant::EntityRoute {
            route: RootIdentifier::Vec.into(),
            kind: EntityKind::Type(TyKind::Vec),
        },
    );
}

#[test]
fn tuple_generics() {
    let mut db = HuskyCompileTime::new_default(__resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "Tuple",
        AtomVariant::EntityRoute {
            route: RootIdentifier::Tuple.into(),
            kind: EntityKind::Type(TyKind::Other),
        },
    );
}
