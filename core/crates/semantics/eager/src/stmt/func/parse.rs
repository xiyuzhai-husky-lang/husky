use std::iter::Peekable;

use vm::{InitKind, XmlTagKind};
use word::IdentPairDict;

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
                        RawStmtVariant::Return(result) => FuncStmtVariant::Return {
                            result: self.parse_eager_expr(result)?,
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
                        RawStmtVariant::ReturnXml(ref raw_xml_expr) => FuncStmtVariant::ReturnXml {
                            xml_expr: self.parse_xml_expr(raw_xml_expr)?,
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
                AstVariant::FeatureDecl { .. } => todo!(),
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
                branches.push(Arc::new(FuncConditionBranch {
                    variant: FuncConditionBranchVariant::If {
                        condition: self.parse_eager_expr(condition)?,
                    },
                    stmts: self.parse_func_stmts(children)?,
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
                        branches.push(Arc::new(FuncConditionBranch {
                            variant: FuncConditionBranchVariant::Else,
                            stmts: self.parse_func_stmts(not_none!(item.opt_children))?,
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
        Ok(FuncStmtVariant::Match {
            match_expr: self.parse_eager_expr(match_expr)?,
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
                            RawPatternBranchVariant::Case { pattern } => FuncPatternBranch {
                                variant: FuncPatternBranchVariant::Case {
                                    pattern: pattern.clone(),
                                },
                                stmts: self.parse_func_stmts(item.opt_children.clone().unwrap())?,
                            },
                            RawPatternBranchVariant::Default => FuncPatternBranch {
                                variant: FuncPatternBranchVariant::Default,
                                stmts: self.parse_func_stmts(item.opt_children.clone().unwrap())?,
                            },
                        })),
                        _ => panic!(),
                    }
                })
                .collect::<SemanticResult<Vec<_>>>()?,
        })
    }

    fn parse_xml_expr(&mut self, raw_xml_expr: &RawXmlExpr) -> SemanticResultArc<XmlExpr> {
        let variant = match raw_xml_expr.variant {
            RawXmlExprVariant::Value(raw_expr_idx) => {
                XmlExprVariant::Value(self.parse_eager_expr(raw_expr_idx)?)
            }
            RawXmlExprVariant::Tag { ident, ref props } => {
                let tag_kind = XmlTagKind::from_ident(ident);
                XmlExprVariant::Tag { tag_kind, props: props
                    .iter()
                    .map(
                        |(ident, raw_expr_idx)| -> SemanticResult<(CustomIdentifier, Arc<EagerExpr>)> {
                            Ok((*ident, self.parse_eager_expr(*raw_expr_idx)?))
                        },
                    )
                    .collect::<SemanticResult<IdentPairDict<Arc<EagerExpr>>>>()? }
            }
        };
        Ok(Arc::new(XmlExpr {
            variant,
            range: raw_xml_expr.range,
            file: self.file,
            instruction_id: Default::default(),
        }))
    }
}
