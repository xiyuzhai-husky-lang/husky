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
    abstract_variable::{ZfsAbstractVariable, ZfsAbstractVariableData},
    abstraction::{ZfsAbstraction, ZfsAbstractionData},
    application::{ZfsApplication, ZfsApplicationData},
    eval::{ZfsEval, ZfsEvalData},
    exists::{ZfsExists, ZfsExistsData},
    forall::{ZfsForAll, ZfsForAllData},
    limit::{ZfsLimit, ZfsLimitData},
    literal::{ZfsLiteral, ZfsLiteralData},
    stack_variable::{ZfsStackVariable, ZfsStackVariableData},
    symbolic_variable::{ZfsSymbolicVariable, ZfsSymbolicVariableData},
};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZfsTerm {
    Literal(ZfsLiteral),
    ForAll(ZfsForAll),
    Exists(ZfsExists),
    Limit(ZfsLimit),
    Eval(ZfsEval),
    SymbolicVariable(ZfsSymbolicVariable),
    AbstractVariable(ZfsAbstractVariable),
    StackVariable(ZfsStackVariable),
    Application(ZfsApplication),
    Abstraction(ZfsAbstraction),
}

pub type ZfsTerms = SmallVec<[ZfsTerm; 4]>;

#[salsa::interned]
pub struct ZfsTermId {
    data: ZfsTermData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZfsTermData {
    Literal(ZfsLiteralData),
    ForAll(ZfsForAllData),
    Exists(ZfsExistsData),
    Limit(ZfsLimitData),
    Eval(ZfsEvalData),
    SymbolicVariable(ZfsSymbolicVariableData),
    AbstractVariable(ZfsAbstractVariableData),
    StackVariable(ZfsStackVariableData),
    Application(ZfsApplicationData),
    Abstraction(ZfsAbstractionData),
}
