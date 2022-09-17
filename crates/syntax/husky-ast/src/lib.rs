mod context;
mod error;
mod expr;
mod field;
mod query;
mod stmt;
mod transform;
mod xml;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use expr::*;
pub use field::*;
use husky_init_syntax::InitKind;
use husky_pattern_syntax::RawPattern;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;
pub use xml::*;

use error::*;
use husky_atom::*;
use husky_check_utils::*;
use husky_defn_head::*;
use husky_dev_utils::*;
use husky_entity_kind::*;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_file::FilePtr;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_text::*;
use husky_word::{CustomIdentifier, IdentDict, Identifier, Paradigm, StmtKeyword};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub variant: AstVariant,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstVariant {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        spatial_parameters: IdentDict<SpatialParameter>,
    },
    MainDefnHead,
    CallFormDefnHead {
        ident: RangedCustomIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        return_ty: RangedEntityRoute,
        output_liason: OutputModifier,
        opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        return_ty: RangedEntityRoute,
    },
    FieldDefnHead {
        liason: MemberModifier,
        ranged_ident: RangedCustomIdentifier,
        field_ty: RangedEntityRoute,
        ast_field_kind: AstFieldKind,
    },
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: RangedCustomIdentifier,
        variant_class: EnumVariantKind,
    },
    Use {
        use_variant: UseVariant,
    },
    Submodule {
        ident: RangedCustomIdentifier,
        source_file: FilePtr,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InferRoot {
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

impl AstVariant {
    pub(crate) fn infer_roots(&self) -> Vec<InferRoot> {
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
                .map(|parameter| InferRoot::Parameter {
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
                    vec![InferRoot::Default { default: *default }]
                }
                AstFieldKind::StructDerivedEager { derivation } => {
                    vec![InferRoot::Derivation {
                        derivation: *derivation,
                    }]
                }
                AstFieldKind::StructProperty { .. } => vec![],
                AstFieldKind::RecordOriginal => todo!(),
                AstFieldKind::RecordDerived => todo!(),
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
                            infer_entrances.push(InferRoot::Bound { bound })
                        }
                        infer_entrances.push(InferRoot::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            infer_entrances.push(InferRoot::Bound { bound })
                        }
                        infer_entrances
                    }
                    RawLoopKind::ForExt {
                        frame_var,
                        final_boundary,
                        step,
                    } => {
                        let mut infer_entrances = vec![];
                        infer_entrances.push(InferRoot::FrameVar { frame_var });
                        if let Some(bound) = final_boundary.opt_bound {
                            infer_entrances.push(InferRoot::Bound { bound })
                        }
                        infer_entrances
                    }
                    RawLoopKind::While { condition } => {
                        vec![InferRoot::Condition { condition }]
                    }
                    RawLoopKind::DoWhile { condition } => {
                        vec![InferRoot::Condition { condition }]
                    }
                },
                RawStmtVariant::IfElseBranch {
                    condition_branch_kind,
                } => match condition_branch_kind {
                    RawConditionBranchKind::If { condition } => {
                        vec![InferRoot::Condition { condition }]
                    }
                    RawConditionBranchKind::Elif { condition } => {
                        vec![InferRoot::Condition { condition }]
                    }
                    RawConditionBranchKind::Else => vec![],
                },
                RawStmtVariant::MatchBranch {
                    ref pattern_branch_variant,
                } => match pattern_branch_variant {
                    RawPatternBranchVariant::Case { pattern } => {
                        vec![InferRoot::Case {
                            pattern: pattern.clone(),
                        }]
                    }
                    RawPatternBranchVariant::Default => vec![],
                },
                RawStmtVariant::Exec { expr, discard } => {
                    vec![InferRoot::Exec { expr, discard }]
                }
                RawStmtVariant::Init {
                    init_kind,
                    varname,
                    initial_value,
                } => vec![InferRoot::Init {
                    init_kind,
                    varname,
                    initial_value,
                }],
                RawStmtVariant::Return {
                    result,
                    return_context,
                } => vec![InferRoot::Return {
                    result,
                    return_context,
                }],
                RawStmtVariant::ReturnXml(ref xml) => vec![InferRoot::Xml { xml: xml.clone() }],
                RawStmtVariant::Assert(condition) => {
                    vec![InferRoot::Condition { condition }]
                }
                RawStmtVariant::Break => vec![],
                RawStmtVariant::Match {
                    match_expr,
                    match_liason,
                } => vec![InferRoot::MatchExpr {
                    match_expr,
                    match_liason,
                }],
                RawStmtVariant::Require {
                    condition,
                    return_context,
                } => vec![
                    InferRoot::Require { return_context },
                    InferRoot::Condition { condition },
                ],
            },
            AstVariant::EnumVariantDefnHead { .. } => vec![],
            AstVariant::Use { .. } => vec![],
            AstVariant::Submodule { .. } => vec![],
            AstVariant::Visual => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { route: EntityRoutePtr },
    All { parent: EntityRoutePtr },
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: AstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstVariant {
    fn from(stmt: RawStmt) -> Self {
        AstVariant::Stmt(stmt)
    }
}
