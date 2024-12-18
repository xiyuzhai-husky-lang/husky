use super::*;
use husky_expr::stmt::{ConditionConversion, LoopBoundaryKind, LoopStep};
use husky_hir_eager_expr::{let_variable::HirEagerLetVariablesPattern, HirEagerCaseBranch};
use husky_hir_opr::suffix::HirSuffixOpr;

impl TranspileToRustWith<HirEagerExprRegion> for (IsLastStmt, HirEagerStmtIdx) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (IsLastStmt(is_last_stmt), slf) = self;
        match *slf.entry(builder.hir_eager_stmt_arena()) {
            HirEagerStmtData::Let {
                pattern,
                initial_value,
                contract,
                coercion,
            } => builder.on_fresh_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Let);
                pattern.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Assign);
                let initial_value_entry = &builder.hir_eager_expr_arena()[initial_value];
                (initial_value, HirEagerExprRole::new_pattern_opd(contract))
                    .transpile_to_rust(builder)
            }),
            HirEagerStmtData::Return { result, coercion } => {
                {
                    let db = builder.db();
                    match builder.hir_eager_expr_region().region_path(db) {
                        husky_entity_path::region::RegionPath::CrateDecl(_) => (),
                        husky_entity_path::region::RegionPath::ItemDecl(_) => (),
                        husky_entity_path::region::RegionPath::ItemDefn(item_path) => {
                            if item_path.ident(db).unwrap().data() == "major_line_segment_sketch" {
                                use husky_print_utils::p;
                                p!(coercion);
                                todo!()
                            }
                        }
                        husky_entity_path::region::RegionPath::Script(_) => (),
                    }
                }
                builder.on_fresh_semicolon_line(|builder| {
                    builder.keyword(RustKeyword::Return);
                    (result, HirEagerExprRole::new_root()).transpile_to_rust(builder)
                })
            }
            HirEagerStmtData::Require { ref condition } => {
                builder.on_fresh_semicolon_line(|builder| {
                    builder.macro_name(RustMacroName::Require);
                    builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                        condition.transpile_to_rust(builder)
                    })
                })
            }
            HirEagerStmtData::Assert { ref condition } => {
                builder.on_fresh_semicolon_line(|builder| match *condition {
                    HirEagerCondition::Be { .. } => todo!(),
                    HirEagerCondition::Other {
                        opd: hir_eager_expr_idx,
                        conversion,
                    } => {
                        builder.macro_name(RustMacroName::Assert);
                        builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                            (hir_eager_expr_idx, HirEagerExprRole::new_root())
                                .transpile_to_rust(builder);
                            match conversion {
                                ConditionConversion::None => (),
                                ConditionConversion::IntToBool(_) => todo!(),
                            }
                        })
                    }
                })
            }
            HirEagerStmtData::Break => {
                builder.on_fresh_semicolon_line(|builder| builder.keyword(RustKeyword::Break))
            }
            HirEagerStmtData::Eval {
                expr,
                coercion,
                discarded,
            } => match discarded || !is_last_stmt {
                true => builder.on_fresh_semicolon_line(|builder| {
                    (expr, HirEagerExprRole::new_root()).transpile_to_rust(builder);
                }),
                false => builder.on_fresh_line(|builder| {
                    (expr, HirEagerExprRole::new_root()).transpile_to_rust(builder);
                }),
            },
            HirEagerStmtData::ForBetween {
                ref particulars,
                for_loop_varible_idx,
                stmts,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::StmtFor);
                particulars
                    .for_loop_variable_ident
                    .transpile_to_rust(builder);
                builder.keyword(RustKeyword::In);
                let range = &particulars.range;
                let t = |opd| {
                    (
                        opd,
                        HirEagerExprRole::subexpr(RustPrecedenceRange::Greater(
                            RustPrecedence::Range,
                        )),
                    )
                };
                match range.step {
                    LoopStep::Constant(step) => match step {
                        1 => {
                            match range.initial_boundary.kind {
                                LoopBoundaryKind::UpperOpen => unreachable!(),
                                LoopBoundaryKind::UpperClosed => unreachable!(),
                                LoopBoundaryKind::LowerOpen => {
                                    match range.initial_boundary.bound_expr {
                                        Some(initial_bound) => {
                                            builder.delimited(RustDelimiter::Par, |builder| {
                                                (
                                                    initial_bound,
                                                    HirEagerExprRole::subexpr(
                                                        RustPrecedenceRange::Greater(
                                                            RustPrecedence::Additive,
                                                        ),
                                                    ),
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
                            builder.delimited(RustDelimiter::Par, |builder| {
                                match range.final_boundary.kind {
                                    LoopBoundaryKind::UpperOpen => unreachable!(),
                                    LoopBoundaryKind::UpperClosed => unreachable!(),
                                    LoopBoundaryKind::LowerOpen => {
                                        match range.final_boundary.bound_expr {
                                            Some(final_bound) => {
                                                builder.delimited(RustDelimiter::Par, |builder| {
                                                    (
                                                        final_bound,
                                                        HirEagerExprRole::subexpr(
                                                            RustPrecedenceRange::Greater(
                                                                RustPrecedence::Additive,
                                                            ),
                                                        ),
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
                            builder.call_rev()
                        }
                        _ => todo!(),
                    },
                }
                stmts.transpile_to_rust(builder)
            }),
            HirEagerStmtData::Forext {
                particulars,
                stmts: block,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::While);
                particulars.forext_loop_var_ident.transpile_to_rust(builder);
                match particulars.boundary_kind {
                    LoopBoundaryKind::UpperOpen => builder.punctuation(RustPunctuation::Less),
                    LoopBoundaryKind::UpperClosed => builder.punctuation(RustPunctuation::Leq),
                    LoopBoundaryKind::LowerOpen => builder.punctuation(RustPunctuation::Greater),
                    LoopBoundaryKind::LowerClosed => builder.punctuation(RustPunctuation::Geq),
                }
                (
                    particulars.bound_expr_hir_eager_expr_idx,
                    HirEagerExprRole::subexpr(RustPrecedenceRange::Greater(
                        RustPrecedence::OrdComparison,
                    )),
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
                stmts: _,
            } => todo!(),
            HirEagerStmtData::While {
                ref condition,
                stmts,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::While);
                condition.transpile_to_rust(builder);
                stmts.transpile_to_rust(builder)
            }),
            HirEagerStmtData::DoWhile {
                ref condition,
                stmts: block,
            } => {
                builder.on_fresh_line(|builder| {
                    builder.keyword(RustKeyword::Loop);
                    builder.curly_block(|builder| {
                        builder.on_fresh_line(|builder| block.transpile_to_rust(builder));
                        builder.on_fresh_line(|builder| {
                            builder.keyword(RustKeyword::If);
                            match *condition {
                                HirEagerCondition::Be { .. } => {
                                    condition.transpile_to_rust(builder)
                                }
                                HirEagerCondition::Other {
                                    opd: hir_eager_expr_idx,
                                    conversion,
                                } => match conversion {
                                    ConditionConversion::None => {
                                        builder.punctuation(RustPunctuation::Not);
                                        (
                                            hir_eager_expr_idx,
                                            HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(
                                                RustPrecedence::Prefix,
                                            )),
                                        )
                                            .transpile_to_rust(builder)
                                    }
                                    ConditionConversion::IntToBool(_) => {
                                        (
                                            hir_eager_expr_idx,
                                            HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(
                                                RustPrecedence::EqComparison,
                                            )),
                                        )
                                            .transpile_to_rust(builder);
                                        // this is because we absorb the outer `!` into this
                                        builder.eq_zero()
                                    }
                                },
                            }
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
                ref if_branch,
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
                opd,
                contract,
                ref case_branches,
            } => builder.on_fresh_line(|builder| {
                builder.keyword(RustKeyword::Match);
                (opd, HirEagerExprRole::new_pattern_opd(contract)).transpile_to_rust(builder);
                builder.delimited_multiline_list(RustDelimiter::Curl, case_branches)
            }),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerCondition {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match *self {
            HirEagerCondition::Be {
                opd,
                contract,
                ref pattern,
            } => {
                builder.keyword(RustKeyword::Let);
                pattern.pattern.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Assign);
                (opd, HirEagerExprRole::new_root()).transpile_to_rust(builder)
            }
            HirEagerCondition::Other { opd, conversion } => match conversion {
                ConditionConversion::None => {
                    (opd, HirEagerExprRole::new_root()).transpile_to_rust(builder)
                }
                ConditionConversion::IntToBool(_) => {
                    (
                        opd,
                        HirEagerExprRole::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::EqComparison,
                        )),
                    )
                        .transpile_to_rust(builder);
                    builder.ne_zero()
                }
            },
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerLetVariablesPattern {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.pattern_idx().transpile_to_rust(builder);
        if let Some(ty) = self.ty() {
            builder.punctuation(RustPunctuation::Colon);
            ty.transpile_to_rust(builder)
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerIfBranch {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerElifBranch {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerElseBranch {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerCaseBranch {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.pattern.transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::HeavyArrow);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerStmtIdxRange {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let end = self.end();
        builder.curly_block(|builder| {
            for stmt in self {
                (IsLastStmt((stmt + 1) == end), stmt).transpile_to_rust(builder)
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IsLastStmt(bool);
