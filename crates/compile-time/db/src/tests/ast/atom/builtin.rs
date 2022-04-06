use super::utils;
use crate::*;
use atom::AtomKind;
use entity_route::{EntityRouteKind, ScopeId};
use word::{Identifier, RootIdentifier};

#[test]
fn std_scope() {
    utils::check_atom_kind(
        "std",
        AtomKind::Scope(RootIdentifier::Std.into(), RawEntityKind::Module),
    );
}

#[test]
fn core_scope() {
    utils::check_atom_kind(
        "core",
        AtomKind::Scope(RootIdentifier::Core.into(), RawEntityKind::Module),
    );
}

#[test]
fn debug_scope() {
    utils::check_atom_kind(
        "debug",
        AtomKind::Scope(RootIdentifier::Debug.into(), RawEntityKind::Module),
    );
}

#[test]
fn i32_type() {
    utils::check_atom_kind(
        "i32",
        AtomKind::Scope(RootIdentifier::I32.into(), RawEntityKind::Type),
    );
}

#[test]
fn f32_type() {
    utils::check_atom_kind(
        "f32",
        AtomKind::Scope(RootIdentifier::F32.into(), RawEntityKind::Type),
    );
}

#[test]
fn vec_generics() {
    utils::check_atom_kind(
        "Vec",
        AtomKind::Scope(RootIdentifier::Vector.into(), RawEntityKind::Type),
    );
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind(
        "Tuple",
        AtomKind::Scope(RootIdentifier::Tuple.into(), RawEntityKind::Type),
    );
}
