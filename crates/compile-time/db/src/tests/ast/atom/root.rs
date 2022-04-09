use super::utils;
use crate::*;
use ast::AtomKind;
use entity_route::{EntityKind, EntityRouteKind, EntityRoutePtr};
use entity_syntax::TyKind;
use word::{Identifier, RootIdentifier};

#[test]
fn std_scope() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "std",
        AtomKind::EntityRoute {
            route: RootIdentifier::Std.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn core_scope() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "core",
        AtomKind::EntityRoute {
            route: RootIdentifier::Core.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn debug_scope() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "debug",
        AtomKind::EntityRoute {
            route: RootIdentifier::Debug.into(),
            kind: EntityKind::Module,
        },
    );
}

#[test]
fn i32_type() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "i32",
        AtomKind::EntityRoute {
            route: RootIdentifier::I32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn f32_type() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "f32",
        AtomKind::EntityRoute {
            route: RootIdentifier::F32.into(),
            kind: EntityKind::Type(TyKind::Primitive),
        },
    );
}

#[test]
fn vec_generics() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "Vec",
        AtomKind::EntityRoute {
            route: RootIdentifier::Vec.into(),
            kind: EntityKind::Type(TyKind::Vec),
        },
    );
}

#[test]
fn tuple_generics() {
    let mut db = HuskyLangCompileTime::default();
    utils::check_atom_kind(
        &mut db,
        "Tuple",
        AtomKind::EntityRoute {
            route: RootIdentifier::Tuple.into(),
            kind: EntityKind::Type(TyKind::Other),
        },
    );
}
