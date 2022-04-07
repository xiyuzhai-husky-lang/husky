use super::utils;
use crate::*;
use ast::AtomKind;
use entity_route::{EntityRouteKind, EntityRoutePtr, RawEntityKind};
use entity_syntax::RawTyKind;
use word::{Identifier, RootIdentifier};

#[test]
fn std_scope() {
    utils::check_atom_kind(
        "std",
        AtomKind::EntityRoute {
            route: RootIdentifier::Std.into(),
            kind: RawEntityKind::Module,
        },
    );
}

#[test]
fn core_scope() {
    utils::check_atom_kind(
        "core",
        AtomKind::EntityRoute {
            route: RootIdentifier::Core.into(),
            kind: RawEntityKind::Module,
        },
    );
}

#[test]
fn debug_scope() {
    utils::check_atom_kind(
        "debug",
        AtomKind::EntityRoute {
            route: RootIdentifier::Debug.into(),
            kind: RawEntityKind::Module,
        },
    );
}

#[test]
fn i32_type() {
    utils::check_atom_kind(
        "i32",
        AtomKind::EntityRoute {
            route: RootIdentifier::I32.into(),
            kind: RawEntityKind::Type(RawTyKind::Primitive),
        },
    );
}

#[test]
fn f32_type() {
    utils::check_atom_kind(
        "f32",
        AtomKind::EntityRoute {
            route: RootIdentifier::F32.into(),
            kind: RawEntityKind::Type(RawTyKind::Primitive),
        },
    );
}

#[test]
fn vec_generics() {
    utils::check_atom_kind(
        "Vec",
        AtomKind::EntityRoute {
            route: RootIdentifier::Vec.into(),
            kind: RawEntityKind::Type(RawTyKind::Primitive),
        },
    );
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind(
        "Tuple",
        AtomKind::EntityRoute {
            route: RootIdentifier::Tuple.into(),
            kind: RawEntityKind::Type(RawTyKind::Primitive),
        },
    );
}
