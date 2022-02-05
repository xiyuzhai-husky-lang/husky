mod impl_a;
mod variable_data;

use arena::Arena;
use ast::{Ast, AstResult, Expr};
use fold::{Executor, LocalStack};

use crate::*;

use variable_data::VariableData;

pub(crate) struct Inferrer<'a> {
    // input
    db: &'a dyn SemanticQuery,
    arena: &'a Arena<Expr>,
    // temp
    variables: LocalStack<VariableData>,
    // output
    data: Vec<Option<ExprData>>,
    errors: Vec<SemanticError>,
}

impl<'a> Inferrer<'a> {
    pub fn new(db: &'a dyn SemanticQuery, arena: &'a Arena<Expr>) -> Self {
        let mut data = Vec::new();
        data.resize(arena.len(), None);
        Self {
            db,
            arena,
            variables: LocalStack::new(),
            data,
            errors: Vec::new(),
        }
    }

    pub(crate) fn finish(self) -> InferenceTable {
        InferenceTable {
            data: self.data,
            errors: self.errors,
        }
    }
}

impl<'a> Executor<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>> for Inferrer<'a> {
    fn _enter_block(&mut self) {
        todo!()
    }

    fn _exit_block(&mut self) {
        todo!()
    }

    fn execute(
        &mut self,
        indent: fold::Indent,
        input: &AstResult<Ast>,
        enter_block: impl FnOnce(&mut Self),
    ) {
        todo!()
    }
}
