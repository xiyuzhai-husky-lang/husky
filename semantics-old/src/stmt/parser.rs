use vm::{StackIdx, VMResult};

use crate::qual::QualTable;

use super::*;

pub(super) struct StmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) variables: Vec<Variable>,
    pub(super) file: FilePtr,
    pub(super) qual_table: QualTable,
}

impl<'a> StmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputPlaceholder],
        db: &'a dyn InferQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> Self {
        Self {
            db,
            arena,
            variables: input_placeholders
                .iter()
                .map(|input_placeholder| Variable::from_input(input_placeholder))
                .collect(),
            file,
            qual_table: Default::default(),
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        ty: ScopePtr,
        qual: Qual,
    ) -> VMResult<StackIdx> {
        let varidx = StackIdx::new(self.variables.len())?;
        self.variables.push(Variable {
            ident: varname,
            ty,
            qual,
        });
        Ok(varidx)
    }

    pub(super) fn varidx(&self, varname: CustomIdentifier) -> StackIdx {
        StackIdx::new(
            self.variables.len()
                - 1
                - self
                    .variables
                    .iter()
                    .rev()
                    .position(|v| v.ident == varname)
                    .unwrap(),
        )
        .unwrap()
    }
}

impl<'a> StrictExprParser<'a> for StmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn vartype(&self, varname: CustomIdentifier) -> ScopePtr {
        self.variables
            .iter()
            .find_map(|variable| {
                if variable.ident == varname {
                    Some(variable.ty)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn db(&self) -> &'a dyn InferQueryGroup {
        self.db
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}
