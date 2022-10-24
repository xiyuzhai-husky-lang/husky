use crate::*;

#[test]
fn new_resource_works() {
    let mut time_machine = TimeMachine::default();
    let immutable0 = time_machine.new_immutable();
    assert_eq!(immutable0.to_string(), "#0");
    let lifetime1 = time_machine.new_lifetime();
    assert_eq!(lifetime1.to_string(), "'#1");
}

#[test]
fn it_works() {
    let mut time_machine = TimeMachine::default();
    let immutable0 = time_machine.new_immutable();
    let lifetime1 = time_machine.new_lifetime();
    assert_eq!(
        time_machine.variable_state(immutable0),
        &VariableState::Intact
    );
    assert_eq!(
        time_machine.lifetime_state(lifetime1),
        LifetimeState::Intact
    );
    time_machine.new_borrow(immutable0, lifetime1);
}
