mod decl;
mod impr;

use common::p;
pub(crate) use decl::gen_decl_stmt_instructions;
pub use decl::{DeclBranchKind, DeclBranchesKind, DeclStmt, DeclStmtKind};
use file::FilePtr;
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

pub(crate) fn parse_decl_stmts(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<DeclStmt>>> {
    let mut parser = DeclStmtParser::new(this, arena, file);
    parser.parse_stmts(iter)
}

pub struct DeclStmtParser<'a> {
    db: &'a dyn InferQueryGroup,
    arena: &'a RawExprArena,
    variables: Vec<Variable>,
    file: FilePtr,
}

pub struct Variable {
    ident: CustomIdentifier,
    ty: ScopePtr,
}

impl<'a> DeclStmtParser<'a> {
    fn new(db: &'a dyn InferQueryGroup, arena: &'a RawExprArena, file: FilePtr) -> Self {
        Self {
            db,
            arena,
            variables: Vec::new(),
            file,
        }
    }

    fn parse_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResultArc<Vec<Arc<DeclStmt>>> {
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
                Ast::Stmt(stmt) => match stmt.kind {
                    RawStmtKind::Loop(_) => todo!(),
                    RawStmtKind::Branch(branch_kind) => {
                        let mut branches = vec![];
                        match branch_kind {
                            RawBranchKind::If { condition } => {
                                branches.push(Arc::new(DeclBranch {
                                    kind: DeclBranchKind::If {
                                        condition: self.parse_expr(&self.arena[condition])?,
                                    },
                                    stmts: self.parse_stmts(not_none!(item.children))?,
                                }))
                            }
                            RawBranchKind::Elif { condition } => todo!(),
                            RawBranchKind::Else => todo!(),
                        }
                        while let Some(item) = iter.peek() {
                            let item = match item.value.as_ref()? {
                                Ast::Stmt(RawStmt {
                                    kind: RawStmtKind::Branch(_),
                                    ..
                                }) => iter.next().unwrap(),
                                _ => break,
                            };
                            match item.value.as_ref()? {
                                Ast::Stmt(RawStmt {
                                    kind: RawStmtKind::Branch(branch_stmt),
                                    ..
                                }) => match branch_stmt {
                                    RawBranchKind::If { condition } => break,
                                    RawBranchKind::Elif { condition } => {
                                        if branches.len() == 0 {
                                            todo!()
                                        }
                                        todo!()
                                    }
                                    RawBranchKind::Else => {
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
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: DeclStmtKind::Branches {
                                kind: DeclBranchesKind::If,
                                branches,
                            },
                        }
                    }
                    RawStmtKind::Exec(_) => todo!(),
                    RawStmtKind::Init {
                        varname,
                        initial_value,
                        ..
                    } => {
                        let initial_value = self.parse_expr(&self.arena[initial_value])?;
                        self.def_variable(varname, initial_value.ty);
                        DeclStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: DeclStmtKind::Init {
                                varname,
                                value: initial_value,
                            },
                        }
                    }
                    RawStmtKind::Return(result) => DeclStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: DeclStmtKind::Return {
                            result: self.parse_expr(&self.arena[result])?,
                        },
                    },
                    RawStmtKind::Assert(condition) => DeclStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: DeclStmtKind::Assert {
                            condition: self.parse_expr(&self.arena[condition])?,
                        },
                    },
                },
            }))
        }
        Ok(Arc::new(stmts))
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

    fn file(&self) -> FilePtr {
        self.file
    }
}
