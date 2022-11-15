#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstEntrance {
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
    pub(crate) fn ast_entrances(&self) -> Vec<AstEntrance> {
        match self {
            AstVariant::TypeDefnHead {
                ident: _,
                kind: _,
                spatial_parameters: _,
            } => vec![],
            AstVariant::MainDefnHead => vec![],
            AstVariant::CallFormDefnHead {
                ident: _,
                paradigm: _,
                spatial_parameters: _,
                parameters,
                return_ty: _,
                output_liason: _,
                opt_this_liason: _,
            } => parameters
                .iter()
                .map(|parameter| AstEntrance::Parameter {
                    parameter: parameter.clone(),
                })
                .collect(),
            AstVariant::FeatureDefnHead {
                paradigm: _,
                ident: _,
                return_ty: _,
            } => vec![],
            AstVariant::FieldDefnHead {
                liason: _,
                ranged_ident: _,
                field_ty: _,
                ast_field_kind,
            } => match ast_field_kind {
                AstFieldKind::StructOriginal => vec![],
                AstFieldKind::StructDefault { default } => {
                    vec![AstEntrance::Default { default: *default }]
                }
                AstFieldKind::StructDerivedEager { derivation } => {
                    vec![AstEntrance::Derivation {
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
                        step: _,
                    } => {
                        let mut infer_entrances = vec![];
                        if let Some(bound) = initial_boundary.opt_bound {
                            infer_entrances.push(AstEntrance::Bound { bound })
                        }
                        infer_entrances.push(AstEntrance::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            infer_entrances.push(AstEntrance::Bound { bound })
                        }
                        infer_entrances
                    }
                    RawLoopKind::ForExt {
                        frame_var,
                        final_boundary,
                        step: _,
                    } => {
                        let mut ast_entrances = vec![];
                        ast_entrances.push(AstEntrance::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            ast_entrances.push(AstEntrance::Bound { bound })
                        }
                        ast_entrances
                    }
                    RawLoopKind::While { condition } => {
                        vec![AstEntrance::Condition { condition }]
                    }
                    RawLoopKind::DoWhile { condition } => {
                        vec![AstEntrance::Condition { condition }]
                    }
                },
                RawStmtVariant::IfElseBranch {
                    condition_branch_kind,
                } => match condition_branch_kind {
                    RawConditionBranchKind::If { condition } => {
                        vec![AstEntrance::Condition { condition }]
                    }
                    RawConditionBranchKind::Elif { condition } => {
                        vec![AstEntrance::Condition { condition }]
                    }
                    RawConditionBranchKind::Else => vec![],
                },
                RawStmtVariant::MatchBranch {
                    ref pattern_branch_variant,
                } => match pattern_branch_variant {
                    RawPatternBranchVariant::Case { pattern } => {
                        vec![AstEntrance::Case {
                            pattern: pattern.clone(),
                        }]
                    }
                    RawPatternBranchVariant::Default => vec![],
                },
                RawStmtVariant::Exec { expr, discard } => {
                    vec![AstEntrance::Exec { expr, discard }]
                }
                RawStmtVariant::Init {
                    init_kind,
                    varname,
                    initial_value,
                } => vec![AstEntrance::Init {
                    init_kind,
                    varname,
                    initial_value,
                }],
                RawStmtVariant::Return {
                    result,
                    return_context,
                } => vec![AstEntrance::Return {
                    result,
                    return_context,
                }],
                RawStmtVariant::ReturnXml(ref xml) => vec![AstEntrance::Xml { xml: xml.clone() }],
                RawStmtVariant::Assert(condition) => {
                    vec![AstEntrance::Condition { condition }]
                }
                RawStmtVariant::Break => vec![],
                RawStmtVariant::Match {
                    match_expr,
                    match_liason,
                } => vec![AstEntrance::MatchExpr {
                    match_expr,
                    match_liason,
                }],
                RawStmtVariant::Require {
                    condition,
                    return_context,
                } => vec![
                    AstEntrance::Require { return_context },
                    AstEntrance::Condition { condition },
                ],
            },
            AstVariant::EnumVariantDefnHead { .. } => vec![],
            AstVariant::Use { .. } => vec![],
            AstVariant::Submodule { .. } => vec![],
            AstVariant::Visual => vec![],
        }
    }
}
