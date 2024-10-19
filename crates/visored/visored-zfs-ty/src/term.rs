pub mod abstract_variable;
pub mod application;
pub mod lambda;
pub mod substitute_variable;
pub mod symbolic_variable;

use self::{
    abstract_variable::{AbstractVariableVisoredZfsTerm, AbstractVariableVisoredZfsTermData},
    application::{ApplicationVisoredZfsTerm, ApplicationVisoredZfsTermData},
    substitute_variable::{SubstituteVariableVisoredZfsTerm, SubstituteVariableVisoredZfsTermData},
    symbolic_variable::{SymbolicVariableVisoredZfsTerm, SymbolicVariableVisoredZfsTermData},
};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredZfsTerm {
    SymbolicVariable(SymbolicVariableVisoredZfsTerm),
    AbstractVariable(AbstractVariableVisoredZfsTerm),
    SubstituteVariable(SubstituteVariableVisoredZfsTerm),
    StackVariable(StackVariableVisoredZfsTerm),
    Application(ApplicationVisoredZfsTerm),
    Abstraction(AbstractionZfsTerm),
}

pub type VisoredZfsTerms = SmallVec<[VisoredZfsTerm; 4]>;

#[salsa::interned]
pub struct VisoredZfsTermId {
    data: VisoredZfsTermData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredZfsTermData {
    SubstituteVariable(SubstituteVariableVisoredZfsTermData),
    FreeVariable(SymbolicVariableVisoredZfsTermData),
    BoundedVariable(AbstractVariableVisoredZfsTermData),
    Application(ApplicationVisoredZfsTermData),
}
