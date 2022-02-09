mod decl;
mod impr;

use common::p;
pub(crate) use decl::gen_decl_stmt_instructions;
pub use decl::{DeclBranchKind, DeclBranchesKind, DeclStmt, DeclStmtKind};
pub use impr::{StrictStmt, StrictStmtKind};

use ast::*;
use scope::{ScopeKind, ScopePtr};
use syntax_types::Opr;
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use crate::SemanticResult;

use crate::error::{err, not_none};
use crate::expr::{BinaryOpnKind, ExprParser, Opn};
use crate::query::infer::InferQueryGroup;
use crate::*;

use self::decl::DeclBranch;

pub(crate) fn parse_lazy_stmts(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
) -> SemanticResult<Vec<Arc<DeclStmt>>> {
    let mut parser = DeclStmtParser::new(this, arena);
    parser.parse_stmts(iter)
}

pub struct DeclStmtParser<'a> {
    db: &'a dyn InferQueryGroup,
    arena: &'a RawExprArena,
    variables: Vec<Variable>,
}

pub struct Variable {
    ident: CustomIdentifier,
    ty: ScopePtr,
}

impl<'a> DeclStmtParser<'a> {
    fn new(db: &'a dyn InferQueryGroup, arena: &'a RawExprArena) -> Self {
        Self {
            db,
            arena,
            variables: Vec::new(),
        }
    }

    fn parse_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<Arc<DeclStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            stmts.push(Arc::new(match item.value.as_ref()? {
                Ast::TypeDef { .. } => todo!(),
                Ast::MainDef => todo!(),
                Ast::DatasetConfig => todo!(),
                Ast::FuncDef { .. } => todo!(),
                Ast::PatternDef => todo!(),
                Ast::Use { .. } => todo!(),
                Ast::MembDef { .. } => todo!(),
                Ast::Stmt(stmt) => match stmt {
                    RawStmt::Loop(_) => todo!(),
                    RawStmt::Branch(stmt) => {
                        let mut branches = vec![];
                        match stmt {
                            BranchRawStmt::If { condition } => {
                                branches.push(Arc::new(DeclBranch {
                                    kind: DeclBranchKind::If {
                                        condition: self.parse_expr(&self.arena[condition])?,
                                    },
                                    stmts: self.parse_stmts(not_none!(item.children))?,
                                }))
                            }
                            BranchRawStmt::Elif { condition } => todo!(),
                            BranchRawStmt::Else => todo!(),
                        }
                        while let Some(item) = iter.peek() {
                            let item = match item.value.as_ref()? {
                                Ast::Stmt(RawStmt::Branch(_)) => iter.next().unwrap(),
                                _ => break,
                            };
                            match item.value.as_ref()? {
                                Ast::Stmt(RawStmt::Branch(branch_stmt)) => match branch_stmt {
                                    BranchRawStmt::If { condition } => break,
                                    BranchRawStmt::Elif { condition } => {
                                        if branches.len() == 0 {
                                            todo!()
                                        }
                                        todo!()
                                    }
                                    BranchRawStmt::Else => {
                                        branches.push(Arc::new(DeclBranch {
                                            kind: DeclBranchKind::Else,
                                            stmts: self.parse_stmts(not_none!(item.children))?,
                                        }));
                                        break;
                                    }
                                },
                                _ => break,
                            }
                        }
                        DeclStmt {
                            kind: DeclStmtKind::Branches {
                                kind: DeclBranchesKind::If,
                                branches,
                            },
                            indent: item.indent,
                        }
                    }
                    RawStmt::Exec(_) => todo!(),
                    RawStmt::Init {
                        varname,
                        initial_value,
                        ..
                    } => {
                        let varname = *varname;
                        let initial_value = self.parse_expr(&self.arena[initial_value])?;
                        self.def_variable(varname, initial_value.ty);
                        DeclStmt {
                            kind: DeclStmtKind::Init {
                                varname,
                                value: initial_value,
                            },
                            indent: item.indent,
                        }
                    }
                    RawStmt::Return(result) => DeclStmt {
                        kind: DeclStmtKind::Return {
                            result: self.parse_expr(&self.arena[result])?,
                        },
                        indent: item.indent,
                    },
                    RawStmt::Assert(condition) => DeclStmt {
                        kind: DeclStmtKind::Assert {
                            condition: self.parse_expr(&self.arena[condition])?,
                        },
                        indent: item.indent,
                    },
                },
            }))
        }
        Ok(stmts)
    }

    fn def_variable(&mut self, varname: CustomIdentifier, ty: ScopePtr) {
        self.variables.push(Variable { ident: varname, ty });
    }
}

impl<'a> ExprParser<'a> for DeclStmtParser<'a> {
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
}
