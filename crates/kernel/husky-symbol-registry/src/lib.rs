mod lifetime;
mod variable;

pub use lifetime::*;
pub use variable::*;

pub enum SymbolIdx {
    Variable(VariableIdx),
    Lifetime(LifetimeIdx),
}

#[derive(Default)]
pub struct SymbolRegistry {
    registrations: Vec<SymbolRegistration>,
}

pub enum SymbolRegistration {
    Variable,
    Lifetime,
}

#[test]
fn it_works() {
    let mut symbol_page = SymbolRegistry::default();
    let immutable0 = symbol_page.new_variable();
    assert_eq!(immutable0.to_string(), "#0");
    let lifetime1 = symbol_page.new_lifetime();
    assert_eq!(lifetime1.to_string(), "'#1");
}
