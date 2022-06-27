use super::utils;
use crate::*;
use husky_atom::AtomVariant;

#[test]
fn list_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32) -> i32");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type2() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn tuple_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32)");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_pointer_with_implicitly_void_return_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Fp(i32, i32)");
    should_eq!(atoms.len(), 1);
    let atom = &atoms[0];
    if let AtomVariant::EntityRoute { route, .. } = atom.kind {
        should_eq!(route.spatial_arguments.len(), 3);
    } else {
        panic!()
    }
}

#[test]
fn func_pointer_with_nonvoid_return_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Fp(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_trait() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Fn(i32, i32)");
    should_eq!(atoms.len(), 1);
}

#[test]
fn list_of_default_func_type_with_nonvoid_return_type() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<(i32, i32) -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn list_of_default_func_type_with_nonvoid_return_type2() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<() -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_list_of_i32() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "[]i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_array_of_i32() {
    let mut db = HuskyCompileTime::new(static_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "[3]i32");
    should_eq!(atoms.len(), 1);
}
