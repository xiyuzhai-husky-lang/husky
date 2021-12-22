use super::utils;
use crate::*;
use atom::AtomKind;
use scope::{BuiltinScope, ScopeId};
use test_utils::assert_test_env;
use word::Identifier;

#[test]
fn vec_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Vec<i32>");
    p!(atoms);
    assert_eq!(atoms.len(), 1);
}

#[test]
fn routine_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("(i32) -> i32");
    p!(atoms);
    assert_eq!(atoms.len(), 1);
}

#[test]
fn tuple_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("(i32, i32)");
    p!(atoms);
    assert_eq!(atoms.len(), 1);
}
