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
fn new_borrow_works() {
    let mut symbol_registry = SymbolRegistry::default();
    let x = symbol_registry.new_variable();
    let a = symbol_registry.new_lifetime();
    let borrows = BorrowTable::default();
    let mut borrow_checker = BorrowChecker::new(&borrows);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    borrow_checker.new_borrow(x, a);
    assert_eq!(borrow_checker.lifetime_state(a), LifetimeState::Intact);
}
