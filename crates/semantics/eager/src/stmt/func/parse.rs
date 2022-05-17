use vm::{EagerContract, InitKind};

use super::parser::EagerStmtParser;
use super::*;
use crate::*;

impl<'a> EagerStmtParser<'a> {
    pub(super) fn parse_decl_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            stmts.push(Arc::new(match item.value.as_ref()?.kind {
                AstKind::Use { .. } => todo!(),
                AstKind::Stmt(ref stmt) => match stmt.variant {
                    RawStmtVariant::Loop(_) => todo!(),
                    RawStmtVariant::ConditionBranch {
                        condition_branch_kind,
                    } => {
                        let mut branches = vec![];
                        match condition_branch_kind {
                            RawConditionBranchKind::If { condition } => {
                                branches.push(Arc::new(DeclBranch {
                                    kind: DeclBranchKind::If {
                                        condition: self.parse_eager_expr(condition)?,
                                    },
                                    stmts: self.parse_decl_stmts(not_none!(item.opt_children))?,
                                }))
                            }
                            RawConditionBranchKind::Elif { condition } => todo!(),
                            RawConditionBranchKind::Else => todo!(),
                        }
                        while let Some(item) = iter.peek() {
                            let item = match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
                                    variant: RawStmtVariant::ConditionBranch { .. },
                                    ..
                                }) => iter.next().unwrap(),
                                _ => break,
                            };
                            match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
                                    variant:
                                        RawStmtVariant::ConditionBranch {
                                            ref condition_branch_kind,
                                        },
                                    ..
                                }) => match condition_branch_kind {
                                    RawConditionBranchKind::If { .. } => break,
                                    RawConditionBranchKind::Elif { condition } => {
                                        if branches.len() == 0 {
                                            todo!()
                                        }
                                        todo!()
                                    }
                                    RawConditionBranchKind::Else => {
                                        branches.push(Arc::new(DeclBranch {
                                            kind: DeclBranchKind::Else,
                                            stmts: self
                                                .parse_decl_stmts(not_none!(item.opt_children))?,
                                        }));
                                        break;
                                    }
                                },
                                _ => break,
                            }
                        }
                        FuncStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            variant: FuncStmtVariant::ConditionFlow { branches },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtVariant::PatternBranch {
                        ref pattern_branch_variant,
                    } => todo!(),
                    RawStmtVariant::Exec(_) => todo!(),
                    RawStmtVariant::Init {
                        varname,
                        initial_value,
                        init_kind: kind,
                    } => {
                        let initial_value = self.parse_eager_expr(initial_value)?;
                        if kind != InitKind::Decl {
                            todo!()
                        }
                        FuncStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            variant: FuncStmtVariant::Init {
                                varname,
                                initial_value,
                            },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtVariant::Return(result) => FuncStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        variant: FuncStmtVariant::Return {
                            result: self.parse_eager_expr(result)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtVariant::Assert(condition) => FuncStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        variant: FuncStmtVariant::Assert {
                            condition: self.parse_eager_expr(condition)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtVariant::Break => todo!(),
                    RawStmtVariant::Match { .. } => todo!(),
                },
                AstKind::FeatureDecl { .. } => todo!(),
                _ => panic!(),
            }))
        }
        Ok(Arc::new(stmts))
    }
}
