pub mod application;
pub mod bounded_variable;
pub mod free_variable;
pub mod substitute_variable;

use self::{
    application::{ApplicationVisoredZfsTerm, ApplicationVisoredZfsTermData},
    bounded_variable::{BoundVariableVisoredZfsTerm, BoundedVariableVisoredZfsTermData},
    free_variable::{FreeVariableVisoredZfsTerm, FreeVariableVisoredZfsTermData},
    substitute_variable::{SubstituteVariableVisoredZfsTerm, SubstituteVariableVisoredZfsTermData},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredZfsTerm {
    SubstituteVariable(SubstituteVariableVisoredZfsTerm),
    FreeVariable(FreeVariableVisoredZfsTerm),
    BoundVariable(BoundVariableVisoredZfsTerm),
    Application(ApplicationVisoredZfsTerm),
}

#[salsa::interned]
pub struct VisoredZfsTermId {
    data: VisoredZfsTermData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredZfsTermData {
    SubstituteVariable(SubstituteVariableVisoredZfsTermData),
    FreeVariable(FreeVariableVisoredZfsTermData),
    BoundedVariable(BoundedVariableVisoredZfsTermData),
    Application(ApplicationVisoredZfsTermData),
}
