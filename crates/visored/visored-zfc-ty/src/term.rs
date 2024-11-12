pub mod abstract_variable;
pub mod abstraction;
pub mod application;
pub mod eval;
pub mod exists;
pub mod forall;
pub mod item_path;
pub mod limit;
pub mod literal;
pub mod stack_variable;
pub mod symbolic_variable;

use self::{
    abstract_variable::{VdZfcAbstractVariable, VdZfcAbstractVariableData},
    abstraction::{VdZfcAbstraction, VdZfcAbstractionData},
    application::{VdZfcApplication, VdZfcApplicationData},
    eval::{VdZfcEval, VdZfcEvalData},
    exists::{VdZfcExists, VdZfcExistsData},
    forall::{VdZfcForAll, VdZfcForAllData},
    item_path::VdZfcItemPath,
    limit::{VdZfcLimit, VdZfcLimitData},
    literal::{VdZfcLiteral, VdZfcLiteralData},
    stack_variable::{VdZfcStackVariable, VdZfcStackVariableData},
    symbolic_variable::{VdZfcSymbolicVariable, VdZfcSymbolicVariableData},
};
use crate::ty::VdZfcType;
use item_path::VdZfcItemPathData;
use smallvec::SmallVec;
use visored_item_path::path::VdItemPath;

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdZfcTerm {
    Literal(VdZfcLiteral),
    ItemPath(VdZfcItemPath),
    ForAll(VdZfcForAll),
    Exists(VdZfcExists),
    Limit(VdZfcLimit),
    Eval(VdZfcEval),
    SymbolicVariable(VdZfcSymbolicVariable),
    AbstractVariable(VdZfcAbstractVariable),
    StackVariable(VdZfcStackVariable),
    Application(VdZfcApplication),
    Abstraction(VdZfcAbstraction),
}

impl std::ops::Deref for VdZfcTerm {
    type Target = VdZfcTermId;
    fn deref(&self) -> &Self::Target {
        match self {
            VdZfcTerm::Literal(literal) => &*literal,
            VdZfcTerm::ItemPath(item_path) => &*item_path,
            VdZfcTerm::ForAll(for_all) => &*for_all,
            VdZfcTerm::Exists(exists) => &*exists,
            VdZfcTerm::Limit(limit) => &*limit,
            VdZfcTerm::Eval(eval) => &*eval,
            VdZfcTerm::SymbolicVariable(symbolic_variable) => &*symbolic_variable,
            VdZfcTerm::AbstractVariable(abstract_variable) => &*abstract_variable,
            VdZfcTerm::StackVariable(stack_variable) => &*stack_variable,
            VdZfcTerm::Application(application) => &*application,
            VdZfcTerm::Abstraction(abstraction) => &*abstraction,
        }
    }
}

pub type ZfcTerms = SmallVec<[VdZfcTerm; 4]>;

#[salsa::interned]
pub struct VdZfcTermId {
    #[return_ref]
    data: VdZfcTermData,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VdZfcTermData {
    Literal(VdZfcLiteralData),
    ItemPath(VdZfcItemPathData),
    ForAll(VdZfcForAllData),
    Exists(VdZfcExistsData),
    Limit(VdZfcLimitData),
    Eval(VdZfcEvalData),
    SymbolicVariable(VdZfcSymbolicVariableData),
    AbstractVariable(VdZfcAbstractVariableData),
    StackVariable(VdZfcStackVariableData),
    Application(VdZfcApplicationData),
    Abstraction(VdZfcAbstractionData),
}

impl VdZfcTerm {
    pub fn to_ty(self, db: &::salsa::Db) -> VdZfcType {
        zfc_term_to_ty(db, *self)
    }
}

#[salsa::tracked]
fn zfc_term_to_ty(db: &::salsa::Db, term_id: VdZfcTermId) -> VdZfcType {
    match *term_id.data(db) {
        VdZfcTermData::Literal(ref data) => todo!(),
        VdZfcTermData::ItemPath(ref data) => VdZfcType::new_item_path(data.item_path(), db),
        VdZfcTermData::ForAll(ref data) => todo!(),
        VdZfcTermData::Exists(ref data) => todo!(),
        VdZfcTermData::Limit(ref data) => todo!(),
        VdZfcTermData::Eval(ref data) => todo!(),
        VdZfcTermData::SymbolicVariable(ref data) => todo!(),
        VdZfcTermData::AbstractVariable(ref data) => todo!(),
        VdZfcTermData::StackVariable(ref data) => todo!(),
        VdZfcTermData::Application(ref data) => todo!(),
        VdZfcTermData::Abstraction(ref data) => todo!(),
    }
}
