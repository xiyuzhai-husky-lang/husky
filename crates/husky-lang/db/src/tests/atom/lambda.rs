use super::utils;
use crate::*;
use test_utils::assert_test_env;

#[test]
fn empty_lambda() {
    let (_, atoms) = utils::get_atoms_in_one_line_group("|| 1");
    assert_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_lambda() {
    let (_, atoms) = utils::get_atoms_in_one_line_group("|x| x");
    assert_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_with_type_lambda() {
    let (_, atoms) = utils::get_atoms_in_one_line_group("|x: i32| x");
    assert_eq!(atoms.len(), 2);
}

#[test]
fn two_arguments_lambda() {
    let (_, atoms) = utils::get_atoms_in_one_line_group("|x: i32, y| x");
    assert_eq!(atoms.len(), 2);
}
