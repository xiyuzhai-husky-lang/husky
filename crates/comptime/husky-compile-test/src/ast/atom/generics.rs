use super::utils;
use crate::*;
use husky_atom::HuskyAtomVariant;

#[test]
fn option_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    should_eq!(utils::get_atoms_in_line(&mut db, "?i32").len(), 1);
    should_eq!(utils::get_atoms_in_line(&mut db, "Option<i32>").len(), 1);
}

#[test]
fn vec_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    should_eq!(utils::get_atoms_in_line(&mut db, "[]i32").len(), 1);
    should_eq!(utils::get_atoms_in_line(&mut db, "Vec<i32>").len(), 1);
}

#[test]
fn default_func_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32) -> i32");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type2() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn tuple_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "(i32, i32)");
    p!(atoms);
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_pointer_with_implicitly_void_output_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "FatFp(i32, i32)");
    should_eq!(atoms.len(), 1);
    let atom = &atoms[0];
    if let HuskyAtomVariant::EntityRoute { route, .. } = atom.variant {
        should_eq!(route.spatial_arguments.len(), 3);
    } else {
        panic!()
    }
}

#[test]
fn func_pointer_with_nonvoid_output_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "FatFp(i32, i32) -> i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn func_trait() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Fn(i32, i32)");
    should_eq!(atoms.len(), 1);
}

#[test]
fn list_of_default_func_type_with_nonvoid_output_type() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<(i32, i32) -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn list_of_default_func_type_with_nonvoid_output_type2() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "Vec<() -> i32>");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_list_of_i32() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "[]i32");
    should_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_array_of_i32() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    let atoms = utils::get_atoms_in_line(&mut db, "[3]i32");
    should_eq!(atoms.len(), 1);
}
