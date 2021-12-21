use super::utils;
use crate::*;
use atom::AtomKind;
use scope::{BuiltinScope, ScopeId};
use test_utils::assert_test_env;
use word::Identifier;

#[test]
fn std_scope() {
    utils::check_atom_kind("std", AtomKind::Scope(BuiltinScope::Std.into()));
}

#[test]
fn core_scope() {
    utils::check_atom_kind("core", AtomKind::Scope(BuiltinScope::Core.into()));
}

#[test]
fn debug_scope() {
    utils::check_atom_kind("debug", AtomKind::Scope(BuiltinScope::Debug.into()));
}

#[test]
fn i32_type() {
    utils::check_atom_kind("i32", AtomKind::Scope(BuiltinScope::I32.into()));
}

#[test]
fn f32_type() {
    utils::check_atom_kind("f32", AtomKind::Scope(BuiltinScope::F32.into()));
}

#[test]
fn vec_generics() {
    utils::check_atom_kind("Vec", AtomKind::Scope(BuiltinScope::Vec.into()));
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind("Tuple", AtomKind::Scope(BuiltinScope::Tuple.into()));
}

#[test]
fn fp_type() {
    utils::check_atom_kind("Fp", AtomKind::Scope(BuiltinScope::Fp.into()));
}

#[test]
fn fn_trait() {
    utils::check_atom_kind("Fn", AtomKind::Scope(BuiltinScope::Fn.into()));
}

#[test]
fn fn_mut_trait() {
    utils::check_atom_kind("FnMut", AtomKind::Scope(BuiltinScope::FnMut.into()));
}

#[test]
fn fn_once_trait() {
    utils::check_atom_kind("FnOnce", AtomKind::Scope(BuiltinScope::FnOnce.into()));
}
