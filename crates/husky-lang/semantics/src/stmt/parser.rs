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
    pub(super) fn new(db: &'a dyn InferQueryGroup, arena: &'a RawExprArena, file: FilePtr) -> Self {
        Self {
            db,
            arena,
            variables: Vec::new(),
            file,
            qual_table: Default::default(),
        }
    }

    pub(super) fn def_variable(&mut self, varname: CustomIdentifier, ty: ScopePtr, qual: Qual) {
        self.variables.push(Variable {
            ident: varname,
            ty,
            qual,
        });
    }
}

impl<'a> ExprParser<'a> for StmtParser<'a> {
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
