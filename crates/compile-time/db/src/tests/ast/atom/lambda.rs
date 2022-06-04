use super::utils;
use crate::*;

#[test]
fn empty_lambda() {
    let mut db = HuskyCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "|| 1");
    should_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_lambda() {
    let mut db = HuskyCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "|x| x");
    should_eq!(atoms.len(), 2);
}

#[test]
fn one_argument_with_type_lambda() {
    let mut db = HuskyCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "|x: i32| x");
    should_eq!(atoms.len(), 2);
}

#[test]
fn two_arguments_lambda() {
    let mut db = HuskyCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "|x: i32, y| x");
    should_eq!(atoms.len(), 2);
}
