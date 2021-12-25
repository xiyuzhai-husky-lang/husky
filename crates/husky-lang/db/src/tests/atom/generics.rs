use super::utils;
use crate::*;
use atom::AtomKind;

#[test]
fn vec_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Vec<i32>");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("(i32) -> i32");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn default_func_type2() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("(i32, i32) -> i32");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn tuple_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("(i32, i32)");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn func_pointer_with_implicitly_void_return_type() {
    let (db, atoms) = utils::get_atoms_in_one_line_group("Fp(i32, i32)");
    assert_eq!(atoms.len(), 1);
    let atom = &atoms[0];
    if let AtomKind::Scope(scope_id, _) = atom.kind {
        let scope = db.id_to_scope(scope_id);
        assert_eq!(scope.args.len(), 3);
    } else {
        panic!()
    }
}

#[test]
fn func_pointer_with_nonvoid_return_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Fp(i32, i32) -> i32");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn func_trait() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Fn(i32, i32)");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn vec_of_default_func_type_with_nonvoid_return_type() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Vec<(i32, i32) -> i32>");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn vec_of_default_func_type_with_nonvoid_return_type2() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("Vec<() -> i32>");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_vec_of_i32() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("[]i32");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn symbolized_array_of_i32() {
    let (_db, atoms) = utils::get_atoms_in_one_line_group("[3]i32");
    assert_eq!(atoms.len(), 1);
}
