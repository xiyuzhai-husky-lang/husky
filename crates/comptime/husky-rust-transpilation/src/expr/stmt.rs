use super::*;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_hir_eager_expr::HirEagerCaseBranch;
use husky_hir_opr::suffix::HirSuffixOpr;

impl TranspileToRust<HirEagerExprRegion> for (IsLastStmt, HirEagerStmtIdx) {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let &(IsLastStmt(is_last_stmt), slf) = self;
        match *slf.data(builder.hir_eager_stmt_arena()) {
            HirEagerStmtData::Let {
                pattern,
                initial_value,
            } => builder.on_fresh_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Let);
                pattern.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Assign);
                any_precedence(initial_value).transpile_to_rust(builder)
            }),
            HirEagerStmtData::Return { result } => builder.on_fresh_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Return);
                any_precedence(result).transpile_to_rust(builder)
            }),
            HirEagerStmtData::Require { condition } => builder.on_fresh_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Require);
                builder.bracketed_list_with(RustBracket::Par, |builder| {
                    any_precedence(condition).transpile_to_rust(builder)
                })
            }),
            HirEagerStmtData::Assert { condition } => builder.on_fresh_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Assert);
                builder.bracketed_list_with(RustBracket::Par, |builder| {
                    any_precedence(condition).transpile_to_rust(builder)
                })
            }),
            HirEagerStmtData::Break => {
                builder.on_fresh_semicolon_line(|builder| builder.keyword(RustKeyword::Break))
            }
            HirEagerStmtData::Eval {
                expr_idx,
                discarded,
            } => match discarded || !is_last_stmt {
                true => builder.on_fresh_semicolon_line(|builder| {
                    any_precedence(expr_idx).transpile_to_rust(builder);
                }),
                false => builder.on_fresh_line(|builder| {
                    any_precedence(expr_idx).transpile_to_rust(builder);
                }),
            },
            HirEagerStmtData::ForBetween {
                ref particulars,
                block,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::StmtFor);
                particulars.frame_var_ident.transpile_to_rust(builder);
                builder.keyword(RustKeyword::In);
                let range = &particulars.range;
                let t = |opd| (RustPrecedenceRange::Greater(RustPrecedence::Range), opd);
                match range.step {
                    LoopStep::Constant(step) => match step {
                        1 => {
                            match range.initial_boundary.kind {
                                LoopBoundaryKind::UpperOpen => unreachable!(),
                                LoopBoundaryKind::UpperClosed => unreachable!(),
                                LoopBoundaryKind::LowerOpen => {
                                    match range.initial_boundary.bound_expr {
                                        Some(initial_bound) => {
                                            builder.bracketed(RustBracket::Par, |builder| {
                                                (
                                                    RustPrecedenceRange::Greater(
                                                        RustPrecedence::Additive,
                                                    ),
                                                    initial_bound,
                                                )
                                                    .transpile_to_rust(builder);
                                                builder.punctuation(RustPunctuation::Add);
                                                builder.one()
                                            })
                                        }
                                        None => unreachable!(),
                                    }
                                }
                                LoopBoundaryKind::LowerClosed => {
                                    match range.initial_boundary.bound_expr {
                                        Some(initial_bound) => {
                                            t(initial_bound).transpile_to_rust(builder)
                                        }
                                        None => builder.zero(),
                                    }
                                }
                            }
                            builder.punctuation(match range.final_boundary.kind {
                                LoopBoundaryKind::UpperOpen => RustPunctuation::DotDot,
                                LoopBoundaryKind::UpperClosed => RustPunctuation::DotDotEq,
                                LoopBoundaryKind::LowerOpen => unreachable!(),
                                LoopBoundaryKind::LowerClosed => unreachable!(),
                            });
                            match range.final_boundary.bound_expr {
                                Some(final_bound) => t(final_bound).transpile_to_rust(builder),
                                None => builder.zero(), // ad hoc, todo: use Default::default()
                            }
                        }
                        -1 => {
                            builder.bracketed(RustBracket::Par, |builder| {
                                match range.final_boundary.kind {
                                    LoopBoundaryKind::UpperOpen => unreachable!(),
                                    LoopBoundaryKind::UpperClosed => unreachable!(),
                                    LoopBoundaryKind::LowerOpen => {
                                        match range.final_boundary.bound_expr {
                                            Some(final_bound) => {
                                                builder.bracketed(RustBracket::Par, |builder| {
                                                    (
                                                        RustPrecedenceRange::Greater(
                                                            RustPrecedence::Additive,
                                                        ),
                                                        final_bound,
                                                    )
                                                        .transpile_to_rust(builder);
                                                    builder.punctuation(RustPunctuation::Add);
                                                    builder.one()
                                                })
                                            }
                                            None => unreachable!(),
                                        }
                                    }
                                    LoopBoundaryKind::LowerClosed => {
                                        match range.final_boundary.bound_expr {
                                            Some(final_bound) => {
                                                t(final_bound).transpile_to_rust(builder)
                                            }
                                            None => builder.zero(),
                                        }
                                    }
                                }
                                builder.punctuation(match range.initial_boundary.kind {
                                    LoopBoundaryKind::UpperOpen => RustPunctuation::DotDot,
                                    LoopBoundaryKind::UpperClosed => RustPunctuation::DotDotEq,
                                    LoopBoundaryKind::LowerOpen => unreachable!(),
                                    LoopBoundaryKind::LowerClosed => unreachable!(),
                                });
                                match range.initial_boundary.bound_expr {
                                    Some(initial_bound) => {
                                        t(initial_bound).transpile_to_rust(builder)
                                    }
                                    None => builder.zero(), // ad hoc, todo: use Default::default()
                                }
                            });
                            builder.call_recv()
                        }
                        _ => todo!(),
                    },
                }
                block.transpile_to_rust(builder)
            }),
            HirEagerStmtData::Forext { particulars, block } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::While);
                particulars.forext_loop_var_ident.transpile_to_rust(builder);
                match particulars.boundary_kind {
                    LoopBoundaryKind::UpperOpen => builder.punctuation(RustPunctuation::Less),
                    LoopBoundaryKind::UpperClosed => builder.punctuation(RustPunctuation::Leq),
                    LoopBoundaryKind::LowerOpen => builder.punctuation(RustPunctuation::Greater),
                    LoopBoundaryKind::LowerClosed => builder.punctuation(RustPunctuation::Geq),
                }
                (
                    RustPrecedenceRange::Greater(RustPrecedence::OrdComparison),
                    particulars.bound_expr_hir_eager_expr_idx,
                )
                    .transpile_to_rust(builder);
                builder.curly_block(|builder| {
                    builder.on_fresh_line(|builder| block.transpile_to_rust(builder));
                    builder.on_fresh_line(|builder| {
                        particulars.forext_loop_var_ident.transpile_to_rust(builder);
                        match particulars.boundary_kind {
                            LoopBoundaryKind::UpperOpen | LoopBoundaryKind::UpperClosed => {
                                HirSuffixOpr::Incr.transpile_to_rust(builder)
                            }
                            LoopBoundaryKind::LowerOpen | LoopBoundaryKind::LowerClosed => {
                                HirSuffixOpr::Decr.transpile_to_rust(builder)
                            }
                        }
                    })
                })
            }),
            HirEagerStmtData::ForIn {
                condition: _,
                block: _,
            } => todo!(),
            HirEagerStmtData::While { condition, stmts } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::While);
                any_precedence(condition).transpile_to_rust(builder);
                stmts.transpile_to_rust(builder)
            }),
            HirEagerStmtData::DoWhile { condition, block } => {
                builder.on_fresh_line(|builder| {
                    builder.keyword(RustKeyword::Loop);
                    builder.curly_block(|builder| {
                        builder.on_fresh_line(|builder| block.transpile_to_rust(builder));
                        builder.on_fresh_line(|builder| {
                            builder.keyword(RustKeyword::If);
                            builder.punctuation(RustPunctuation::Not);
                            (RustPrecedenceRange::Geq(RustPrecedence::Prefix), condition)
                                .transpile_to_rust(builder);
                            builder.curly_block(|builder| {
                                builder.on_fresh_semicolon_line(|builder| {
                                    builder.keyword(RustKeyword::Break)
                                })
                            })
                        })
                    })
                })
                // block.transpile_to_rust(builder)
            }
            HirEagerStmtData::IfElse {
                if_branch,
                ref elif_branches,
                else_branch,
            } => builder.on_fresh_line(|builder| {
                if_branch.transpile_to_rust(builder);
                for elif_branch in elif_branches {
                    elif_branch.transpile_to_rust(builder)
                }
                else_branch.transpile_to_rust(builder)
            }),
            HirEagerStmtData::Match {
                match_target,
                ref case_branches,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::Match);
                any_precedence(match_target).transpile_to_rust(builder);
                builder.bracketed_multiline_list(RustBracket::Curl, case_branches)
            }),
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for (RustPrecedenceRange, HirEagerCondition) {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        // todo: do proper conversion
        let &(precedence, condition) = self;
        (precedence, condition.hir_eager_expr_idx()).transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerLetVariablesPattern {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.pattern_expr_idx().transpile_to_rust(builder);
        if let Some(ty) = self.ty() {
            builder.punctuation(RustPunctuation::Colon);
            ty.transpile_to_rust(builder)
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerIfBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::If);
        any_precedence(self.condition).transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerElifBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        builder.keyword(RustKeyword::If);
        any_precedence(self.condition).transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerElseBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerCaseBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.pattern.transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::HeavyArrow);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerStmtIdxRange {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let end = self.end();
        builder.curly_block(|builder| {
            for stmt in self {
                (IsLastStmt((stmt + 1) == end), stmt).transpile_to_rust(builder)
            }
        })
    }
}

struct IsLastStmt(bool);
