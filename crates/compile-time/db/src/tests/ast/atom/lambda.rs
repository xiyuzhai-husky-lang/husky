use super::utils;
use crate::*;

#[test]
fn empty_lambda() {
    let (_, atoms) = utils::get_stmt_atoms_in_one_line_group("|| 1");
    should_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_lambda() {
    let (_, atoms) = utils::get_stmt_atoms_in_one_line_group("|x| x");
    should_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_with_type_lambda() {
    let (_, atoms) = utils::get_stmt_atoms_in_one_line_group("|x: i32| x");
    should_eq!(atoms.len(), 2);
}

#[test]
fn two_arguments_lambda() {
    let (_, atoms) = utils::get_stmt_atoms_in_one_line_group("|x: i32, y| x");
    should_eq!(atoms.len(), 2);
}
