use std::iter::Peekable;

use super::{parser::EagerStmtParser, *};
use crate::*;

type IterType<'a> = fold::FoldIter<'a, AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>;

impl<'a> EagerStmtParser<'a> {
    fn parse_boundary(&mut self, boundary: RawBoundary) -> SemanticResult<Boundary> {
        let bound = if let Some(bound) = boundary.opt_bound {
            Some(self.parse_eager_expr(bound)?)
        } else {
            None
        };
        Ok(Boundary {
            opt_bound: bound,
            kind: boundary.kind,
        })
    }

    pub(super) fn parse_proc_stmts(
        &mut self,
        iter: IterType,
    ) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            let instruction_id = InstructionId::default();
            stmts.push(Arc::new(match item.value.as_ref()?.variant {
                AstKind::TypeDefnHead { .. } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::RoutineDefnHead { .. } => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::Use { .. } => todo!(),
                AstKind::Stmt(ref stmt) => ProcStmt {
                    file: self.file,
                    range: stmt.range,
                    indent: item.indent,
                    variant: self.parse_proc_stmt(stmt, item.opt_children, &mut iter)?,
                    instruction_id,
                },
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: ref variant_kind,
                } => todo!(),
                AstKind::FieldDefnHead { .. } => todo!(),
                AstKind::TypeMethodDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { .. } => todo!(),
                AstKind::Submodule { ident, source_file } => todo!(),
                AstKind::TypeAssociatedRoutineDefnHead(_) => todo!(),
                AstKind::Visual => todo!(),
            }))
        }
        Ok(Arc::new(stmts))
    }

    fn parse_proc_stmt(
        &mut self,
        stmt: &RawStmt,
        children: Option<IterType>,
        iter: &mut Peekable<IterType>,
    ) -> SemanticResult<ProcStmtVariant> {
        match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => self.parse_loop_stmt(loop_kind, not_none!(children)),
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => self.parse_proc_condition_flow(
                stmt,
                not_none!(children),
                iter,
                condition_branch_kind,
            ),
            RawStmtVariant::Exec { expr, silent } => {
                let expr = self.parse_eager_expr(expr)?;
                if !silent && expr.ty != EntityRoutePtr::Root(RootIdentifier::Void) {
                    err!(format!(
                        "expect non-silent executed expression to be of type void, but got {:?} instead",
                        expr.ty
                    ))
                }
                Ok(ProcStmtVariant::Execute { expr })
            }
            RawStmtVariant::Init {
                varname,
                initial_value,
                init_kind,
            } => Ok(ProcStmtVariant::Init {
                varname,
                initial_value: self.parse_eager_expr(initial_value)?,
                init_kind,
            }),
            RawStmtVariant::Return(result) => Ok(ProcStmtVariant::Return {
                result: self.parse_eager_expr(result)?,
            }),
            RawStmtVariant::Assert(condition) => Ok(ProcStmtVariant::Assert {
                condition: self.parse_eager_expr(condition)?,
            }),
            RawStmtVariant::Break => Ok(ProcStmtVariant::Break),
            RawStmtVariant::Match {
                match_expr,
                match_contract,
            } => self.parse_proc_match(stmt, not_none!(children), match_expr, match_contract),
            RawStmtVariant::PatternBranch { .. } => {
                panic!("pattern branch must be inside match stmt")
            }
            RawStmtVariant::ReturnXml(_) => todo!(),
        }
    }

    fn parse_proc_condition_flow(
        &mut self,
        stmt: &RawStmt,
        children: IterType,
        iter: &mut Peekable<AstIter>,
        condition_branch_kind: RawConditionBranchKind,
    ) -> SemanticResult<ProcStmtVariant> {
        let mut branches = vec![];
        match condition_branch_kind {
            RawConditionBranchKind::If { condition } => {
                branches.push(Arc::new(ProcConditionBranch {
                    variant: ProcConditionBranchVariant::If {
                        condition: self.parse_eager_expr(condition)?,
                    },
                    stmts: self.parse_proc_stmts(children)?,
                    range: stmt.range,
                    file: self.file,
                }))
            }
            RawConditionBranchKind::Elif { condition } => todo!(),
            RawConditionBranchKind::Else => todo!(),
        }
        while let Some(item) = iter.peek() {
            let item = match item.value.as_ref()?.variant {
                AstKind::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            ref condition_branch_kind,
                        },
                    ..
                }) => match condition_branch_kind {
                    RawConditionBranchKind::If { .. } => break,
                    RawConditionBranchKind::Elif { .. } | RawConditionBranchKind::Else => {
                        iter.next().unwrap()
                    }
                },
                _ => break,
            };
            match item.value.as_ref()?.variant {
                AstKind::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind,
                        },
                    ..
                }) => match condition_branch_kind {
                    RawConditionBranchKind::If { .. } => panic!(),
                    RawConditionBranchKind::Elif { condition } => {
                        if branches.len() == 0 {
                            todo!()
                        }
                        branches.push(Arc::new(ProcConditionBranch {
                            variant: ProcConditionBranchVariant::Elif {
                                condition: self.parse_eager_expr(condition)?,
                            },
                            stmts: self.parse_proc_stmts(not_none!(item.opt_children))?,
                            range: stmt.range,
                            file: self.file,
                        }));
                    }
                    RawConditionBranchKind::Else => {
                        branches.push(Arc::new(ProcConditionBranch {
                            variant: ProcConditionBranchVariant::Else,
                            stmts: self.parse_proc_stmts(not_none!(item.opt_children))?,
                            range: stmt.range,
                            file: self.file,
                        }));
                        break;
                    }
                },
                _ => break,
            }
        }
        Ok(ProcStmtVariant::ConditionFlow { branches })
    }

    fn parse_loop_stmt(
        &mut self,
        loop_kind: RawLoopKind,
        children: AstIter,
    ) -> SemanticResult<ProcStmtVariant> {
        Ok(match loop_kind {
            RawLoopKind::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => ProcStmtVariant::Loop {
                loop_variant: LoopVariant::For {
                    frame_var,
                    initial_boundary: self.parse_boundary(initial_boundary)?,
                    final_boundary: self.parse_boundary(final_boundary)?,
                    step,
                },
                stmts: self.parse_proc_stmts(children)?,
            },
            RawLoopKind::ForExt {
                frame_var,
                final_boundary,
                step,
            } => {
                emsg_once!("todo: change frame var qual in forext");
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::ForExt {
                        frame_var,
                        final_boundary: self.parse_boundary(final_boundary)?,
                        step,
                    },
                    stmts: self.parse_proc_stmts(children)?,
                }
            }
            RawLoopKind::While { condition } => {
                let condition = self.parse_eager_expr(condition)?;
                match condition.ty {
                    EntityRoutePtr::Root(RootIdentifier::Bool)
                    | EntityRoutePtr::Root(RootIdentifier::I32)
                    | EntityRoutePtr::Root(RootIdentifier::F32)
                    | EntityRoutePtr::Root(RootIdentifier::B32)
                    | EntityRoutePtr::Root(RootIdentifier::B64) => (),
                    _ => todo!(),
                }
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::While { condition },
                    stmts: self.parse_proc_stmts(children)?,
                }
            }
            RawLoopKind::DoWhile { condition } => {
                let condition = self.parse_eager_expr(condition)?;
                match condition.ty {
                    EntityRoutePtr::Root(RootIdentifier::Bool)
                    | EntityRoutePtr::Root(RootIdentifier::I32)
                    | EntityRoutePtr::Root(RootIdentifier::F32)
                    | EntityRoutePtr::Root(RootIdentifier::B32)
                    | EntityRoutePtr::Root(RootIdentifier::B64) => (),
                    _ => todo!(),
                }
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::DoWhile { condition },
                    stmts: self.parse_proc_stmts(children)?,
                }
            }
        })
    }

    fn parse_proc_match(
        &mut self,
        stmt: &RawStmt,
        children: AstIter,
        match_expr: RawExprIdx,
        match_contract: MatchContract,
    ) -> SemanticResult<ProcStmtVariant> {
        Ok(ProcStmtVariant::Match {
            match_expr: self.parse_eager_expr(match_expr)?,
            branches: children
                .map(|item| {
                    let value = item.value.as_ref().unwrap();
                    match value.variant {
                        AstKind::Stmt(RawStmt {
                            variant:
                                RawStmtVariant::PatternBranch {
                                    ref pattern_branch_variant,
                                },
                            range,
                        }) => Ok(Arc::new(match pattern_branch_variant {
                            RawPatternBranchVariant::Case { pattern } => ProcPatternBranch {
                                variant: ProcPatternBranchVariant::Case {
                                    pattern: pattern.clone(),
                                },
                                stmts: self.parse_proc_stmts(item.opt_children.clone().unwrap())?,
                                range,
                                file: self.file,
                            },
                            RawPatternBranchVariant::Default => ProcPatternBranch {
                                variant: ProcPatternBranchVariant::Default,
                                stmts: self.parse_proc_stmts(item.opt_children.clone().unwrap())?,
                                range,
                                file: self.file,
                            },
                        })),
                        _ => panic!(),
                    }
                })
                .collect::<SemanticResult<Vec<_>>>()?,
        })
    }
}
