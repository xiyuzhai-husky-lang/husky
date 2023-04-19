mod lifetime;
mod variable;

pub use self::lifetime::*;
pub use self::variable::*;

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
    let mut symbol_region = SymbolRegistry::default();
    let immutable0 = symbol_region.new_variable();
    assert_eq!(immutable0.to_string(), "#0");
    let lifetime1 = symbol_region.new_lifetime();
    assert_eq!(lifetime1.to_string(), "'#1");
}
