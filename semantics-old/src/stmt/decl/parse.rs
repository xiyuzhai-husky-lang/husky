use vm::{Contract, InitKind};

use super::parser::StmtParser;
use super::*;
use crate::*;

impl<'a> StmtParser<'a> {
    pub(super) fn parse_decl_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResultArc<Vec<Arc<DeclStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            stmts.push(Arc::new(match item.value.as_ref()?.kind {
                AstKind::TypeDef { .. } => todo!(),
                AstKind::MainDef => todo!(),
                AstKind::DatasetConfig => todo!(),
                AstKind::RoutineDef { .. } => todo!(),
                AstKind::PatternDef => todo!(),
                AstKind::Use { .. } => todo!(),
                AstKind::MembDef { .. } => todo!(),
                AstKind::Stmt(ref stmt) => match stmt.kind {
                    RawStmtKind::Loop(_) => todo!(),
                    RawStmtKind::Branch(branch_kind) => {
                        let mut branches = vec![];
                        match branch_kind {
                            RawBranchKind::If { condition } => {
                                branches.push(Arc::new(DeclBranch {
                                    kind: DeclBranchKind::If {
                                        condition: self.parse_eager_expr(
                                            &self.arena[condition],
                                            Contract::PureInput,
                                        )?,
                                    },
                                    stmts: self.parse_decl_stmts(not_none!(item.children))?,
                                }))
                            }
                            RawBranchKind::Elif { condition } => todo!(),
                            RawBranchKind::Else => todo!(),
                        }
                        while let Some(item) = iter.peek() {
                            let item = match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
                                    kind: RawStmtKind::Branch(_),
                                    ..
                                }) => iter.next().unwrap(),
                                _ => break,
                            };
                            match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
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
                                            stmts: self
                                                .parse_decl_stmts(not_none!(item.children))?,
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
                                kind: DeclBranchGroupKind::If,
                                branches,
                            },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtKind::Exec(_) => todo!(),
                    RawStmtKind::Init {
                        varname,
                        initial_value,
                        init_kind: kind,
                    } => {
                        let initial_value =
                            self.parse_eager_expr(&self.arena[initial_value], Contract::Take)?;
                        if kind != InitKind::Decl {
                            todo!()
                        }
                        let qual = Qual::from_init(kind);
                        self.def_variable(varname, initial_value.ty, qual);
                        DeclStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: DeclStmtKind::Init {
                                varname,
                                value: initial_value,
                            },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtKind::Return(result) => DeclStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: DeclStmtKind::Return {
                            result: self.parse_eager_expr(&self.arena[result], Contract::Take)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtKind::Assert(condition) => DeclStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: DeclStmtKind::Assert {
                            condition: self
                                .parse_eager_expr(&self.arena[condition], Contract::PureInput)?,
                        },
                        instruction_id: Default::default(),
                    },
                },
            }))
        }
        Ok(Arc::new(stmts))
    }
}
