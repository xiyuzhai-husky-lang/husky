use super::utils;
use crate::*;
use atom::AtomKind;

#[test]
fn vec_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "(i32) -> i32");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type2() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn tuple_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32)");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_pointer_with_implicitly_void_return_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Fp(i32, i32)");
    should_eq!(atoms.len(), 1);
    let atom = &atoms[0];
    if let AtomKind::EntityRoute { route, .. } = atom.kind {
        should_eq!(route.generics.len(), 3);
    } else {
        panic!()
    }
}

#[test]
fn func_pointer_with_nonvoid_return_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Fp(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_trait() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Fn(i32, i32)");
    should_eq!(atoms.len(), 1);
}

#[test]
fn vec_of_default_func_type_with_nonvoid_return_type() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<(i32, i32) -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn vec_of_default_func_type_with_nonvoid_return_type2() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<() -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_vec_of_i32() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "[]i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_array_of_i32() {
    let mut db = HuskyLangCompileTime::default();
    let atoms = utils::get_atoms_in_line(&mut db, "[3]i32");
    should_eq!(atoms.len(), 1);
}
