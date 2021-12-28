use super::utils;
use crate::*;
use atom::AtomKind;

#[test]
fn some_type_with_lifetime() {
    // this is wrong semantically, but should be okay during atomization phase
    let (_, atoms) = utils::get_stmt_atoms_in_one_line_group("Vec<'a>");
    assert_eq!(atoms.len(), 1);
}

#[test]
fn some_type_with_lifetime_and_generics1() {
    // this is wrong semantically, but should be okay during atomization phase
    let (db, atoms) = utils::get_stmt_atoms_in_one_line_group("Vec<'a, 'b, i32>");
    assert_eq!(atoms.len(), 1);
    if let AtomKind::Scope(id, _kind) = atoms[0].kind {
        let scope = db.id_to_scope(id);
        assert_eq!(scope.lifetimes.len(), 2);
        assert_eq!(scope.generics.len(), 1);
    } else {
        panic!()
    }
}

#[test]
fn some_type_with_lifetime_and_generics2() {
    // this is wrong semantically, but should be okay during atomization phase
    let (db, atoms) = utils::get_stmt_atoms_in_one_line_group("Vec<'a, 'b, i32,>");
    assert_eq!(atoms.len(), 1);
    if let AtomKind::Scope(id, _kind) = atoms[0].kind {
        let scope = db.id_to_scope(id);
        assert_eq!(scope.lifetimes.len(), 2);
        assert_eq!(scope.generics.len(), 1);
    } else {
        panic!()
    }
}
