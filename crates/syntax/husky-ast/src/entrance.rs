#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprEntrance {
    Condition {
        condition: RawExprIdx,
    },
    MatchExpr {
        match_expr: RawExprIdx,
        match_liason: MatchLiason,
    },
    Case {
        pattern: RawPattern,
    },
    Bound {
        bound: RawExprIdx,
    },
    Init {
        init_kind: InitKind,
        varname: RangedCustomIdentifier,
        initial_value: RawExprIdx,
    },
    FrameVar {
        frame_var: RangedCustomIdentifier,
    },
    Parameter {
        parameter: Parameter,
    },
    Require {
        return_context: RawReturnContext,
    },
    Return {
        result: RawExprIdx,
        return_context: RawReturnContext,
    },
    Exec {
        expr: RawExprIdx,
        discard: bool,
    },
    Default {
        default: RawExprIdx,
    },
    Derivation {
        derivation: RawExprIdx,
    },
    Xml {
        xml: Arc<RawXmlExpr>,
    },
}

use crate::*;

impl AstVariant {
    // todo: change this to SmallVec
    pub(crate) fn expr_entrances(&self) -> Vec<ExprEntrance> {
        match self {
            AstVariant::TypeDefnHead {
                ident,
                kind,
                spatial_parameters,
            } => vec![],
            AstVariant::MainDefnHead => vec![],
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                spatial_parameters,
                parameters,
                return_ty,
                output_liason,
                opt_this_liason,
            } => parameters
                .iter()
                .map(|parameter| ExprEntrance::Parameter {
                    parameter: parameter.clone(),
                })
                .collect(),
            AstVariant::FeatureDefnHead {
                paradigm,
                ident,
                return_ty,
            } => vec![],
            AstVariant::FieldDefnHead {
                liason,
                ranged_ident,
                field_ty,
                ast_field_kind,
            } => match ast_field_kind {
                AstFieldKind::StructOriginal => vec![],
                AstFieldKind::StructDefault { default } => {
                    vec![ExprEntrance::Default { default: *default }]
                }
                AstFieldKind::StructDerivedEager { derivation } => {
                    vec![ExprEntrance::Derivation {
                        derivation: *derivation,
                    }]
                }
                AstFieldKind::StructProperty { .. } => vec![],
                AstFieldKind::RecordOriginal => vec![],
                AstFieldKind::RecordDerived => vec![],
            },
            AstVariant::DatasetConfigDefnHead => vec![],
            AstVariant::Stmt(stmt) => match stmt.variant {
                RawStmtVariant::Loop(loop_kind) => match loop_kind {
                    RawLoopKind::For {
                        frame_var,
                        initial_boundary,
                        final_boundary,
                        step,
                    } => {
                        let mut infer_entrances = vec![];
                        if let Some(bound) = initial_boundary.opt_bound {
                            infer_entrances.push(ExprEntrance::Bound { bound })
                        }
                        infer_entrances.push(ExprEntrance::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            infer_entrances.push(ExprEntrance::Bound { bound })
                        }
                        infer_entrances
                    }
                    RawLoopKind::ForExt {
                        frame_var,
                        final_boundary,
                        step,
                    } => {
                        let mut expr_entrances = vec![];
                        expr_entrances.push(ExprEntrance::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            expr_entrances.push(ExprEntrance::Bound { bound })
                        }
                        expr_entrances
                    }
                    RawLoopKind::While { condition } => {
                        vec![ExprEntrance::Condition { condition }]
                    }
                    RawLoopKind::DoWhile { condition } => {
                        vec![ExprEntrance::Condition { condition }]
                    }
                },
                RawStmtVariant::IfElseBranch {
                    condition_branch_kind,
                } => match condition_branch_kind {
                    RawConditionBranchKind::If { condition } => {
                        vec![ExprEntrance::Condition { condition }]
                    }
                    RawConditionBranchKind::Elif { condition } => {
                        vec![ExprEntrance::Condition { condition }]
                    }
                    RawConditionBranchKind::Else => vec![],
                },
                RawStmtVariant::MatchBranch {
                    ref pattern_branch_variant,
                } => match pattern_branch_variant {
                    RawPatternBranchVariant::Case { pattern } => {
                        vec![ExprEntrance::Case {
                            pattern: pattern.clone(),
                        }]
                    }
                    RawPatternBranchVariant::Default => vec![],
                },
                RawStmtVariant::Exec { expr, discard } => {
                    vec![ExprEntrance::Exec { expr, discard }]
                }
                RawStmtVariant::Init {
                    init_kind,
                    varname,
                    initial_value,
                } => vec![ExprEntrance::Init {
                    init_kind,
                    varname,
                    initial_value,
                }],
                RawStmtVariant::Return {
                    result,
                    return_context,
                } => vec![ExprEntrance::Return {
                    result,
                    return_context,
                }],
                RawStmtVariant::ReturnXml(ref xml) => vec![ExprEntrance::Xml { xml: xml.clone() }],
                RawStmtVariant::Assert(condition) => {
                    vec![ExprEntrance::Condition { condition }]
                }
                RawStmtVariant::Break => vec![],
                RawStmtVariant::Match {
                    match_expr,
                    match_liason,
                } => vec![ExprEntrance::MatchExpr {
                    match_expr,
                    match_liason,
                }],
                RawStmtVariant::Require {
                    condition,
                    return_context,
                } => vec![
                    ExprEntrance::Require { return_context },
                    ExprEntrance::Condition { condition },
                ],
            },
            AstVariant::EnumVariantDefnHead { .. } => vec![],
            AstVariant::Use { .. } => vec![],
            AstVariant::Submodule { .. } => vec![],
            AstVariant::Visual => vec![],
        }
    }
}
