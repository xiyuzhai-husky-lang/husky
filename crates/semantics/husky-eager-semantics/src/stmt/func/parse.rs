use std::iter::Peekable;

use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_word::IdentPairDict;

use super::parser::EagerParser;
use super::*;
use crate::*;

impl<'a> EagerParser<'a> {
    pub(super) fn parse_func_stmts(
        &mut self,
        iter: AstIter,
    ) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            match item.value.as_ref().unwrap().variant {
                AstVariant::Use { .. } => todo!(),
                AstVariant::Stmt(ref stmt) => {
                    let variant = match stmt.variant {
                        RawStmtVariant::Loop(_) => todo!(),
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind,
                        } => self.parse_func_condition_flow(
                            stmt,
                            not_none!(item.opt_children),
                            &mut iter,
                            condition_branch_kind,
                        )?,
                        RawStmtVariant::PatternBranch {
                            ref pattern_branch_variant,
                        } => todo!(),
                        RawStmtVariant::Exec { .. } => todo!(),
                        RawStmtVariant::Init {
                            varname,
                            initial_value,
                            init_kind: kind,
                        } => FuncStmtVariant::Init {
                            varname,
                            initial_value: self.parse_eager_expr(initial_value)?,
                        },
                        RawStmtVariant::Return {
                            result,
                            return_context,
                        } => FuncStmtVariant::Return {
                            result: self.parse_eager_expr(result)?,
                            return_context,
                        },
                        RawStmtVariant::Assert(condition) => FuncStmtVariant::Assert {
                            condition: self.parse_eager_expr(condition)?,
                        },
                        RawStmtVariant::Break => todo!(),
                        RawStmtVariant::Match {
                            match_expr,
                            match_liason: match_contract,
                        } => self.parse_func_match(
                            stmt,
                            not_none!(item.opt_children),
                            match_expr,
                            match_contract,
                        )?,
                        RawStmtVariant::ReturnXml(_) => panic!(),
                        RawStmtVariant::Require {
                            condition,
                            return_context,
                        } => FuncStmtVariant::Require {
                            condition: self.parse_eager_expr(condition)?,
                            return_context,
                        },
                    };
                    stmts.push(Arc::new(FuncStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        variant,
                        instruction_id: Default::default(),
                    }))
                }
                AstVariant::FeatureDefnHead { .. } => todo!(),
                _ => panic!(),
            }
        }
        Ok(Arc::new(stmts))
    }

    fn parse_func_condition_flow(
        &mut self,
        stmt: &RawStmt,
        children: AstIter,
        iter: &mut Peekable<AstIter>,
        condition_branch_kind: RawConditionBranchKind,
    ) -> SemanticResult<FuncStmtVariant> {
        let mut branches = vec![];
        match condition_branch_kind {
            RawConditionBranchKind::If { condition } => {
                branches.push(Arc::new(FuncConditionFlowBranch {
                    variant: FuncConditionFlowBranchVariant::If {
                        condition: self.parse_eager_expr(condition)?,
                    },
                    stmts: self.parse_func_stmts(children)?,
                    range: stmt.range,
                    file: self.file,
                    idx: 0,
                }))
            }
            RawConditionBranchKind::Elif { condition } => todo!(),
            RawConditionBranchKind::Else => todo!(),
        }
        while let Some(item) = iter.peek() {
            let item = match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
                    variant: RawStmtVariant::ConditionBranch { .. },
                    ..
                }) => iter.next().unwrap(),
                _ => break,
            };
            match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
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
                        branches.push(Arc::new(FuncConditionFlowBranch {
                            variant: FuncConditionFlowBranchVariant::Else,
                            stmts: self.parse_func_stmts(not_none!(item.opt_children))?,
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
        Ok(FuncStmtVariant::ConditionFlow { branches })
    }

    fn parse_func_match(
        &mut self,
        stmt: &RawStmt,
        children: AstIter,
        match_expr: RawExprIdx,
        match_contract: MatchLiason,
    ) -> SemanticResult<FuncStmtVariant> {
        let match_expr = self.parse_eager_expr(match_expr)?;
        Ok(FuncStmtVariant::Match {
            branches: children
                .map(|item| {
                    let value = item.value.as_ref().unwrap();
                    match value.variant {
                        AstVariant::Stmt(RawStmt {
                            variant:
                                RawStmtVariant::PatternBranch {
                                    ref pattern_branch_variant,
                                },
                            ..
                        }) => Ok(Arc::new(match pattern_branch_variant {
                            RawPatternBranchVariant::Case { pattern } => FuncStmtPatternBranch {
                                variant: FuncStmtPatternBranchVariant::Case {
                                    pattern: self.parse_func_pattern(pattern, match_expr.ty())?,
                                },
                                stmts: self.parse_func_stmts(item.opt_children.clone().unwrap())?,
                            },
                            RawPatternBranchVariant::Default => FuncStmtPatternBranch {
                                variant: FuncStmtPatternBranchVariant::Default,
                                stmts: self.parse_func_stmts(item.opt_children.clone().unwrap())?,
                            },
                        })),
                        _ => panic!(),
                    }
                })
                .collect::<SemanticResult<Vec<_>>>()?,
            match_expr,
        })
    }

    fn parse_func_pattern(
        &mut self,
        raw_pattern: &RawPattern,
        ty: EntityRoutePtr,
    ) -> SemanticResult<FuncStmtPattern> {
        let variant = match raw_pattern.variant {
            RawPatternVariant::PrimitiveLiteral(data) => {
                FuncStmtPatternVariant::PrimitiveLiteral(data)
            }
            RawPatternVariant::OneOf { ref subpatterns } => FuncStmtPatternVariant::OneOf {
                subpatterns: subpatterns
                    .iter()
                    .map(|raw_pattern| self.parse_func_pattern(raw_pattern, ty))
                    .collect::<SemanticResult<_>>()?,
            },
            RawPatternVariant::EnumLiteral(route) => FuncStmtPatternVariant::EnumLiteral(route),
            RawPatternVariant::Some => todo!(),
            RawPatternVariant::None => todo!(),
        };
        Ok(FuncStmtPattern { ty, variant })
    }
}
