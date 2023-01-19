use crate::*;

#[test]
fn new_resource_works() {
    let mut symbol_page = SymbolRegistry::default();
    let _immutable0 = symbol_page.new_variable();
    let _lifetime1 = symbol_page.new_lifetime();
    let borrow_table = DependencyTable::default();
    let _time_machine = BorrowChecker::new(&borrow_table);
}

#[test]
fn new_borrow_works() {
    let mut symbol_page = SymbolRegistry::default();
    let x = symbol_page.new_variable();
    let a = symbol_page.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(borrow_checker.lifetime_state(a), LifetimeState::Intact);
}

fn multiple_borrows() {
    let mut symbol_page = SymbolRegistry::default();
    let x = symbol_page.new_variable();
    let a = symbol_page.new_lifetime();
    let b = symbol_page.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    dependencies.add_borrow(x, b);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    borrow_checker.exec(&BorrowInstruction::push(b));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(borrow_checker.sim_borrow(x, b), Ok(()));
    assert_eq!(borrow_checker.lifetime_state(a), LifetimeState::Intact);
    assert_eq!(borrow_checker.lifetime_state(b), LifetimeState::Intact);
}

#[test]
fn borrow_then_borrow_mut() {
    let mut symbol_page = SymbolRegistry::default();
    let x = symbol_page.new_variable();
    let a = symbol_page.new_lifetime();
    let b = symbol_page.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    dependencies.add_borrow(x, b);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    borrow_checker.exec(&BorrowInstruction::push(b));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(borrow_checker.sim_borrow_mut(x, b), Ok(()));
}

#[test]
fn new_borrow_err() {
    let mut symbol_page = SymbolRegistry::default();
    let x = symbol_page.new_variable();
    let a = symbol_page.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(
        borrow_checker.sim_borrow_mut(x, a),
        Err(BorrowError::InvalidLifetime)
    );
}

#[test]
fn move_err() {
    let mut symbol_page = SymbolRegistry::default();
    let x = symbol_page.new_variable();
    let a = symbol_page.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    borrow_checker.sim_move(x);
    assert_eq!(borrow_checker.lifetime_state(a), LifetimeState::Outdated);
}
