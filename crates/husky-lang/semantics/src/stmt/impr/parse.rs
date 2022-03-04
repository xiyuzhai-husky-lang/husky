use super::{parser::StmtParser, *};
use crate::{qual::QualTable, *};

impl<'a> StmtParser<'a> {
    pub(super) fn parse_impr_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResultArc<Vec<Arc<ImprStmt>>> {
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
                                branches.push(Arc::new(ImprBranch {
                                    kind: ImprBranchKind::If {
                                        condition: self.parse_expr(&self.arena[condition])?,
                                    },
                                    stmts: self.parse_impr_stmts(not_none!(item.children))?,
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
                                        branches.push(Arc::new(ImprBranch {
                                            kind: ImprBranchKind::Else,
                                            stmts: self
                                                .parse_impr_stmts(not_none!(item.children))?,
                                        }));
                                        break;
                                    }
                                },
                                _ => break,
                            }
                        }
                        ImprStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: ImprStmtKind::BranchGroup {
                                kind: ImprBranchGroupKind::If,
                                branches,
                            },
                        }
                    }
                    RawStmtKind::Exec(expr) => {
                        let expr = self.parse_expr(&self.arena[expr])?;
                        if expr.ty != ScopePtr::Builtin(BuiltinIdentifier::Void) {
                            err!(format!("expect executed expression to be of type void, but got {:?} instead", expr.ty))
                        }
                        todo!()
                    }
                    RawStmtKind::Init {
                        varname,
                        initial_value,
                        kind,
                    } => {
                        let initial_value = self.parse_expr(&self.arena[initial_value])?;
                        let qual = Qual::from_init(kind, &mut self.qual_table);
                        self.def_variable(varname, initial_value.ty, qual);
                        ImprStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: ImprStmtKind::Init {
                                varname,
                                initial_value,
                            },
                        }
                    }
                    RawStmtKind::Return(result) => ImprStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: ImprStmtKind::Return {
                            result: self.parse_expr(&self.arena[result])?,
                        },
                    },
                    RawStmtKind::Assert(condition) => ImprStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: ImprStmtKind::Assert {
                            condition: self.parse_expr(&self.arena[condition])?,
                        },
                    },
                },
            }))
        }
        Ok(Arc::new(stmts))
    }
}
