use super::utils;
use crate::*;
use atom::AtomKind;
use scope::{ScopeId, ScopeKind};
use test_utils::assert_test_env;
use word::{BuiltinIdentifier, Identifier};

#[test]
fn std_scope() {
    utils::check_atom_kind(
        "std",
        AtomKind::Scope(BuiltinIdentifier::Std.into(), ScopeKind::Module),
    );
}

#[test]
fn core_scope() {
    utils::check_atom_kind(
        "core",
        AtomKind::Scope(BuiltinIdentifier::Core.into(), ScopeKind::Module),
    );
}

#[test]
fn debug_scope() {
    utils::check_atom_kind(
        "debug",
        AtomKind::Scope(BuiltinIdentifier::Debug.into(), ScopeKind::Module),
    );
}

#[test]
fn i32_type() {
    utils::check_atom_kind(
        "i32",
        AtomKind::Scope(BuiltinIdentifier::I32.into(), ScopeKind::Module),
    );
}

#[test]
fn f32_type() {
    utils::check_atom_kind(
        "f32",
        AtomKind::Scope(BuiltinIdentifier::F32.into(), ScopeKind::Module),
    );
}

#[test]
fn vec_generics() {
    utils::check_atom_kind(
        "Vec",
        AtomKind::Scope(BuiltinIdentifier::Vector.into(), ScopeKind::Module),
    );
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind(
        "Tuple",
        AtomKind::Scope(BuiltinIdentifier::Tuple.into(), ScopeKind::Module),
    );
}

#[test]
fn fp_type() {
    utils::check_atom_kind(
        "Fp",
        AtomKind::Scope(BuiltinIdentifier::Fp.into(), ScopeKind::Type),
    );
}

#[test]
fn fn_trait() {
    utils::check_atom_kind(
        "Fn",
        AtomKind::Scope(BuiltinIdentifier::Fn.into(), ScopeKind::Trait),
    );
}

#[test]
fn fn_mut_trait() {
    utils::check_atom_kind(
        "FnMut",
        AtomKind::Scope(BuiltinIdentifier::FnMut.into(), ScopeKind::Module),
    );
}

#[test]
fn fn_once_trait() {
    utils::check_atom_kind(
        "FnOnce",
        AtomKind::Scope(BuiltinIdentifier::FnOnce.into(), ScopeKind::Module),
    );
}
