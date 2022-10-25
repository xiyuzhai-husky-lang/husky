use crate::*;

#[test]
fn new_resource_works() {
    let mut symbol_registry = SymbolRegistry::default();
    let immutable0 = symbol_registry.new_variable();
    let lifetime1 = symbol_registry.new_lifetime();
    let mut borrow_table = BorrowTable::default();
    let mut time_machine = BorrowChecker::new(&borrow_table);
}

#[test]
fn it_works() {
    let mut symbol_registry = SymbolRegistry::default();
    let immutable0 = symbol_registry.new_variable();
    let lifetime1 = symbol_registry.new_lifetime();
    let borrows = BorrowTable::default();
    let mut borrow_checker = BorrowChecker::new(&borrows);
    borrow_checker.exec(&BorrowInstruction::init(immutable0));
    borrow_checker.exec(&BorrowInstruction::init(lifetime1));
    assert_eq!(
        borrow_checker.variable_state(immutable0),
        &VariableState::Intact
    );
    assert_eq!(borrow_checker.lifetime_state(lifetime1), None);
    borrow_checker.new_borrow(immutable0, lifetime1);
}
