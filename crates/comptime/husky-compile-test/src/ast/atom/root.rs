#[cfg(test)]
use super::utils;
#[cfg(test)]
use crate::*;
#[cfg(test)]
use husky_atom::*;
#[cfg(test)]
use husky_entity_kind::TyKind;
#[cfg(test)]
use husky_entity_route::EntityKind;
#[cfg(test)]
use husky_word::RootIdentifier;

#[test]
fn std_scope() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "std",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::Std.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn core_scope() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "core",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::Core.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn debug_scope() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "debug",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::Debug.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn i32_type() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "i32",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::I32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn i64_type() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "i64",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::I64.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn f32_type() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "f32",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::F32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn f64_type() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "f64",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::F64.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn vec_generics() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "Vec",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::Vec.into(),
            kind: EntityKind::Type(TyKind::Vec),
        },
    );
}

#[test]
fn tuple_generics() {
    let mut db = HuskyComptime::new_default("haha".into(), __resolve_root_defn);
    utils::check_atom_kind(
        &mut db,
        "Tuple",
        HuskyAtomVariant::EntityRoute {
            route: RootIdentifier::Tuple.into(),
            kind: EntityKind::Type(TyKind::Tuple),
        },
    );
}
