mod lazy;
mod strict;

pub use lazy::{LazyStmt, LazyStmtKind};
pub use strict::{StrictStmt, StrictStmtKind};

use ast::*;
use scope::{ScopeId, ScopeKind};
use syntax_types::{BinaryOpr, Opr};
use text::TextRange;
use word::{CustomIdentifier, ReservedIdentifier};

use crate::SemanticResult;

use crate::error::err;
use crate::expr::{BinaryOpnKind, ExprParser, Opn};
use crate::query::signature::CallSignatureQueryGroup;
use crate::*;

pub trait LazyStmtQueryGroup: CallSignatureQueryGroup {
    fn as_lazy_stmt_query_group(&self) -> &dyn LazyStmtQueryGroup;

    fn parse_lazy_stmts(
        &self,
        arena: &RawExprArena,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<LazyStmt>> {
        let mut parser = LazyStmtParser::new(self.as_lazy_stmt_query_group(), arena);
        parser.parse_stmt(iter)
    }
}

pub struct LazyStmtParser<'a> {
    db: &'a dyn LazyStmtQueryGroup,
    arena: &'a RawExprArena,
    variables: Vec<Variable>,
}

pub struct Variable {
    ident: CustomIdentifier,
    ty: ScopeId,
}

impl<'a> LazyStmtParser<'a> {
    fn new(db: &'a dyn LazyStmtQueryGroup, arena: &'a RawExprArena) -> Self {
        Self {
            db,
            arena,
            variables: Vec::new(),
        }
    }

    fn parse_stmt(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<LazyStmt>> {
        iter.map(|item| {
            Ok(match item.value.as_ref()? {
                Ast::TypeDef { .. } => todo!(),
                Ast::MainDef => todo!(),
                Ast::DatasetConfig => todo!(),
                Ast::FuncDef { .. } => todo!(),
                Ast::PatternDef => todo!(),
                Ast::Use { .. } => todo!(),
                Ast::MembDef { .. } => todo!(),
                Ast::Stmt(stmt) => match stmt {
                    RawStmt::Loop(_) => todo!(),
                    RawStmt::Branch(_) => todo!(),
                    RawStmt::Exec(_) => todo!(),
                    RawStmt::Init {
                        varname,
                        initial_value,
                        ..
                    } => {
                        let varname = *varname;
                        let initial_value = self.parse_expr(&self.arena[initial_value])?;
                        self.def_variable(varname, initial_value.ty);
                        LazyStmt {
                            kind: LazyStmtKind::Init {
                                varname,
                                initial_value,
                            },
                        }
                    }
                    RawStmt::Return(result) => LazyStmt {
                        kind: LazyStmtKind::Return {
                            result: self.parse_expr(&self.arena[result])?,
                        },
                    },
                    RawStmt::Assert(condition) => LazyStmt {
                        kind: LazyStmtKind::Assert {
                            condition: self.parse_expr(&self.arena[condition])?,
                        },
                    },
                },
            })
        })
        .collect()
    }

    fn def_variable(&mut self, varname: CustomIdentifier, ty: ScopeId) {
        self.variables.push(Variable { ident: varname, ty });
    }
}

impl<'a> ExprParser<'a> for LazyStmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn vartype(&self, varname: CustomIdentifier) -> ScopeId {
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

    fn db(&self) -> &'a dyn LazyStmtQueryGroup {
        self.db
    }
}
