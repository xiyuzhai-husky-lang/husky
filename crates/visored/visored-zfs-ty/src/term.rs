pub mod abstract_variable;
pub mod abstraction;
pub mod application;
pub mod eval;
pub mod exists;
pub mod forall;
pub mod limit;
pub mod literal;
pub mod stack_variable;
pub mod symbolic_variable;

use self::{
    abstract_variable::{VdZfsAbstractVariable, VdZfsAbstractVariableData},
    abstraction::{VdZfsAbstraction, VdZfsAbstractionData},
    application::{VdZfsApplication, VdZfsApplicationData},
    eval::{VdZfsEval, VdZfsEvalData},
    exists::{VdZfsExists, VdZfsExistsData},
    forall::{VdZfsForAll, VdZfsForAllData},
    limit::{VdZfsLimit, VdZfsLimitData},
    literal::{VdZfsLiteral, VdZfsLiteralData},
    stack_variable::{VdZfsStackVariable, VdZfsStackVariableData},
    symbolic_variable::{VdZfsSymbolicVariable, VdZfsSymbolicVariableData},
};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdZfsTerm {
    Literal(VdZfsLiteral),
    ForAll(VdZfsForAll),
    Exists(VdZfsExists),
    Limit(VdZfsLimit),
    Eval(VdZfsEval),
    SymbolicVariable(VdZfsSymbolicVariable),
    AbstractVariable(VdZfsAbstractVariable),
    StackVariable(VdZfsStackVariable),
    Application(VdZfsApplication),
    Abstraction(VdZfsAbstraction),
}

pub type ZfsTerms = SmallVec<[VdZfsTerm; 4]>;

#[salsa::interned]
pub struct VdZfsTermId {
    #[return_ref]
    data: VdZfsTermData,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdZfsTermData {
    Literal(VdZfsLiteralData),
    ForAll(VdZfsForAllData),
    Exists(VdZfsExistsData),
    Limit(VdZfsLimitData),
    Eval(VdZfsEvalData),
    SymbolicVariable(VdZfsSymbolicVariableData),
    AbstractVariable(VdZfsAbstractVariableData),
    StackVariable(VdZfsStackVariableData),
    Application(VdZfsApplicationData),
    Abstraction(VdZfsAbstractionData),
}
