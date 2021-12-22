use super::utils;
use crate::*;
use atom::AtomKind;
use scope::{BuiltinScope, ScopeId, ScopeKind};
use test_utils::assert_test_env;
use word::Identifier;

#[test]
fn std_scope() {
    utils::check_atom_kind(
        "std",
        AtomKind::Scope(BuiltinScope::Std.into(), ScopeKind::Module),
    );
}

#[test]
fn core_scope() {
    utils::check_atom_kind(
        "core",
        AtomKind::Scope(BuiltinScope::Core.into(), ScopeKind::Module),
    );
}

#[test]
fn debug_scope() {
    utils::check_atom_kind(
        "debug",
        AtomKind::Scope(BuiltinScope::Debug.into(), ScopeKind::Module),
    );
}

#[test]
fn i32_type() {
    utils::check_atom_kind(
        "i32",
        AtomKind::Scope(BuiltinScope::I32.into(), ScopeKind::Module),
    );
}

#[test]
fn f32_type() {
    utils::check_atom_kind(
        "f32",
        AtomKind::Scope(BuiltinScope::F32.into(), ScopeKind::Module),
    );
}

#[test]
fn vec_generics() {
    utils::check_atom_kind(
        "Vec",
        AtomKind::Scope(BuiltinScope::Vec.into(), ScopeKind::Module),
    );
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind(
        "Tuple",
        AtomKind::Scope(BuiltinScope::Tuple.into(), ScopeKind::Module),
    );
}

#[test]
fn fp_type() {
    utils::check_atom_kind(
        "Rp",
        AtomKind::Scope(BuiltinScope::Rp.into(), ScopeKind::Module),
    );
}

#[test]
fn fn_trait() {
    utils::check_atom_kind(
        "Rt",
        AtomKind::Scope(BuiltinScope::Rt.into(), ScopeKind::Module),
    );
}

#[test]
fn fn_mut_trait() {
    utils::check_atom_kind(
        "RtMut",
        AtomKind::Scope(BuiltinScope::RtMut.into(), ScopeKind::Module),
    );
}

#[test]
fn fn_once_trait() {
    utils::check_atom_kind(
        "RtOnce",
        AtomKind::Scope(BuiltinScope::RtOnce.into(), ScopeKind::Module),
    );
}
