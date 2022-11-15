use std::iter::Peekable;

use husky_expr_syntax::RawExprIdx;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};

use super::{parser::EagerParser, *};
use crate::*;

impl<'a> EagerParser<'a> {
    fn parse_boundary(&mut self, boundary: RawBoundary) -> SemanticResult<Boundary> {
        let bound = if let Some(bound) = boundary.opt_bound {
            Some(self.parse_eager_expr(bound, None)?)
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
        iter: AstIter,
    ) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            let instruction_id = InstructionId::default();
            stmts.push(Arc::new(match item.value.as_ref().unwrap().variant {
                AstVariant::TypeDefnHead { .. } => todo!(),
                AstVariant::MainDefnHead => todo!(),
                AstVariant::DatasetConfigDefnHead => todo!(),
                AstVariant::CallFormDefnHead { .. } => todo!(),
                AstVariant::Use { .. } => todo!(),
                AstVariant::Stmt(ref stmt) => {
                    let variant = self.parse_proc_stmt(stmt, item.opt_children, &mut iter)?;
                    ProcStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        instruction_id,
                        variant,
                    }
                }
                AstVariant::EnumVariantDefnHead { .. } => todo!(),
                AstVariant::FieldDefnHead { .. } => todo!(),
                AstVariant::FeatureDefnHead { .. } => todo!(),
                AstVariant::Submodule { .. } => todo!(),
                AstVariant::Visual => todo!(),
            }))
        }
        Ok(Arc::new(stmts))
    }

    fn parse_proc_stmt(
        &mut self,
        stmt: &RawStmt,
        children: Option<AstIter>,
        iter: &mut Peekable<AstIter>,
    ) -> SemanticResult<ProcStmtVariant> {
        match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => self.parse_loop_stmt(loop_kind, not_none!(children)),
            RawStmtVariant::IfElseBranch {
                condition_branch_kind,
            } => self.parse_proc_condition_flow(
                stmt,
                not_none!(children),
                iter,
                condition_branch_kind,
            ),
            RawStmtVariant::Exec {
                expr,
                discard: silent,
            } => {
                let expr = self.parse_eager_expr(expr, None)?;
                if !silent
                    && expr.intrinsic_ty() != EntityRouteItd::Root(RootBuiltinIdentifier::Void)
                {
                    err!(format!(
                        "expect non-silent executed expression to be of type void, but got {:?} instead",
                        expr.intrinsic_ty()
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
                initial_value: self.parse_eager_expr(initial_value, None)?,
                init_kind,
            }),
            RawStmtVariant::Return {
                result,
                return_context,
            } => Ok(ProcStmtVariant::Return {
                result: self.parse_eager_expr(result, None)?, // todo: unveil
                return_context,
            }),
            RawStmtVariant::Assert(condition) => Ok(ProcStmtVariant::Assert {
                condition: self.parse_eager_expr(condition, None)?,
            }),
            RawStmtVariant::Break => Ok(ProcStmtVariant::Break),
            RawStmtVariant::Match {
                match_expr,
                match_liason: match_contract,
            } => self.parse_proc_match(stmt, not_none!(children), match_expr, match_contract),
            RawStmtVariant::MatchBranch { .. } => {
                panic!("pattern branch must be inside match stmt")
            }
            RawStmtVariant::ReturnXml(_) => todo!(),
            RawStmtVariant::Require { .. } => todo!(),
        }
    }

    fn parse_proc_condition_flow(
        &mut self,
        stmt: &RawStmt,
        children: AstIter,
        iter: &mut Peekable<AstIter>,
        condition_branch_kind: RawConditionBranchKind,
    ) -> SemanticResult<ProcStmtVariant> {
        let mut branches = vec![];
        match condition_branch_kind {
            RawConditionBranchKind::If { condition } => {
                branches.push(Arc::new(ProcConditionFlowBranch {
                    variant: ProcConditionFlowBranchVariant::If {
                        condition: self.parse_eager_expr(condition, None)?,
                    },
                    stmts: self.parse_proc_stmts(children)?,
                    range: stmt.range,
                    file: self.file,
                    idx: 0,
                }))
            }
            RawConditionBranchKind::Elif { condition: _ } => todo!(),
            RawConditionBranchKind::Else => todo!(),
        }
        while let Some(item) = iter.peek() {
            let item = match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::IfElseBranch {
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
            match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::IfElseBranch {
                            condition_branch_kind,
                        },
                    ..
                }) => match condition_branch_kind {
                    RawConditionBranchKind::If { .. } => panic!(),
                    RawConditionBranchKind::Elif { condition } => {
                        if branches.len() == 0 {
                            todo!()
                        }
                        branches.push(Arc::new(ProcConditionFlowBranch {
                            variant: ProcConditionFlowBranchVariant::Elif {
                                condition: self.parse_eager_expr(condition, None)?,
                            },
                            stmts: self.parse_proc_stmts(not_none!(item.opt_children))?,
                            range: stmt.range,
                            file: self.file,
                            idx: branches.len().try_into().unwrap(),
                        }));
                    }
                    RawConditionBranchKind::Else => {
                        branches.push(Arc::new(ProcConditionFlowBranch {
                            variant: ProcConditionFlowBranchVariant::Else,
                            stmts: self.parse_proc_stmts(not_none!(item.opt_children))?,
                            range: stmt.range,
                            file: self.file,
                            idx: branches.len().try_into().unwrap(),
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
                msg_once!("todo: change frame var qual in forext");
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
                let condition = self.parse_eager_expr(condition, None)?;
                match condition.intrinsic_ty() {
                    EntityRouteItd::Root(RootBuiltinIdentifier::Bool)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::I32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::F32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::B32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::B64) => (),
                    _ => todo!(),
                }
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::While { condition },
                    stmts: self.parse_proc_stmts(children)?,
                }
            }
            RawLoopKind::DoWhile { condition } => {
                let condition = self.parse_eager_expr(condition, None)?;
                match condition.intrinsic_ty() {
                    EntityRouteItd::Root(RootBuiltinIdentifier::Bool)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::I32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::F32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::B32)
                    | EntityRouteItd::Root(RootBuiltinIdentifier::B64) => (),
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
        _stmt: &RawStmt,
        _children: AstIter,
        _match_expr: RawExprIdx,
        _match_contract: MatchLiason,
    ) -> SemanticResult<ProcStmtVariant> {
        todo!()
        // let match_expr = self.parse_eager_expr(match_expr, None)?;
        // Ok(ProcStmtVariant::Match {
        //     branches: children
        //         .map(|item| {
        //             let value = item.value.as_ref().unwrap();
        //             match value.variant {
        //                 AstVariant::Stmt(RawStmt {
        //                     variant:
        //                         RawStmtVariant::MatchBranch {
        //                             ref pattern_branch_variant,
        //                         },
        //                     range,
        //                 }) => Ok(Arc::new(match pattern_branch_variant {
        //                     RawPatternBranchVariant::Case { pattern } => ProcStmtPatternBranch {
        //                         variant: ProcStmtPatternBranchVariant::Case {
        //                             pattern: self
        //                                 .parse_proc_pattern(pattern, match_expr.intrinsic_ty())?,
        //                         },
        //                         stmts: self.parse_proc_stmts(item.opt_children.clone().unwrap())?,
        //                         range,
        //                         file: self.file,
        //                     },
        //                     RawPatternBranchVariant::Default => ProcStmtPatternBranch {
        //                         variant: ProcStmtPatternBranchVariant::Default,
        //                         stmts: self.parse_proc_stmts(item.opt_children.clone().unwrap())?,
        //                         range,
        //                         file: self.file,
        //                     },
        //                 })),
        //                 _ => panic!(),
        //             }
        //         })
        //         .collect::<SemanticResult<Vec<_>>>()?,
        //     match_expr,
        // })
    }

    fn parse_proc_pattern(
        &mut self,
        raw_pattern: &RawPattern,
        ty: EntityRouteItd,
    ) -> SemanticResult<ProcStmtPattern> {
        let variant = match raw_pattern.variant {
            RawPatternVariant::PrimitiveLiteral(data) => {
                ProcStmtPatternVariant::PrimitiveLiteral(data)
            }
            RawPatternVariant::OneOf { ref subpatterns } => ProcStmtPatternVariant::OneOf {
                subpatterns: subpatterns
                    .iter()
                    .map(|raw_pattern| self.parse_proc_pattern(raw_pattern, ty))
                    .collect::<SemanticResult<_>>()?,
            },
            RawPatternVariant::EnumLiteral(route) => ProcStmtPatternVariant::EnumLiteral(route),
            RawPatternVariant::Some => todo!(),
            RawPatternVariant::None => todo!(),
        };
        Ok(ProcStmtPattern { ty, variant })
    }
}

impl<'a> ParseEagerExpr<'a> for EagerParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        todo!()
    }

    fn file(&self) -> FileItd {
        todo!()
    }

    fn target_entrance(&self) -> FileItd {
        todo!()
    }
}
