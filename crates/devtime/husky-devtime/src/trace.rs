mod impl_func_stmt;
mod impl_proc_stmt;

use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_word::Identifier;

use super::*;

impl HuskyDevtime {
    pub(crate) fn feature_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureLazyStmt>,
    ) -> Vec<TraceId> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. }
            | FeatureLazyStmtVariant::Assert { .. }
            | FeatureLazyStmtVariant::Require { .. }
            | FeatureLazyStmtVariant::Return { .. }
            | FeatureLazyStmtVariant::ReturnUnveil { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureStmt(stmt),
                )]
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
            FeatureLazyStmtVariant::ReturnXml { .. } => todo!(),
        }
    }

    pub(crate) fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureLazyBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureBranch(branch),
        )
    }

    pub(crate) fn new_eager_expr_trace(
        &mut self,
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> TraceId {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
        )
    }

    pub(crate) fn new_call_head_trace(
        &mut self,
        parent: &Trace,
        entity: Arc<EntityDefn>,
    ) -> TraceId {
        let tokens = match entity.variant {
            EntityDefnVariant::Func { ref parameters, .. } => routine_call_head_tokens(
                &self.runtime().text(entity.file).unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Proc { ref parameters, .. } => routine_call_head_tokens(
                &self.runtime().text(entity.file).unwrap(),
                "proc ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Module { .. } => todo!(),
            EntityDefnVariant::Feature { .. } => todo!(),
            EntityDefnVariant::Function { .. } => todo!(),
            EntityDefnVariant::Method { ref parameters, .. } => routine_call_head_tokens(
                &self.runtime().text(entity.file).unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Ty { .. } => todo!(),
            EntityDefnVariant::Trait { .. } => todo!(),
            EntityDefnVariant::EnumVariant { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        };
        return self.new_trace(
            Some(parent.id()),
            0,
            TraceVariant::CallHead { entity, tokens },
        );

        fn routine_call_head_tokens<'eval>(
            text: &HuskyText,
            routine_keyword: &'static str,
            ident: Identifier,
            parameters: &[Parameter],
        ) -> Vec<TraceTokenData> {
            todo!()
            // let mut tokens = vec![
            //     keyword!(routine_keyword),
            //     ident!(ident.as_str()),
            //     trace_token_special!("("),
            // ];
            // for i in 0..parameters.len() {
            //     let parameter = &parameters[i];
            //     match parameter.liason() {
            //         ParameterModifier::None => (),
            //         ParameterModifier::Owned => todo!(),
            //         ParameterModifier::OwnedMut => todo!(),
            //         ParameterModifier::MemberAccess => todo!(),
            //         ParameterModifier::EvalRef => todo!(),
            //         ParameterModifier::TempRef => todo!(),
            //         ParameterModifier::TempRefMut => todo!(),
            //     }
            //     tokens.gen_ident_token(parameter.ident().as_str()));
            //     tokens.push(trace_token_special!(": "));
            //     tokens.push(route!(text.ranged(parameter.raw_ty_range())));
            //     if i < parameters.len() - 1 {
            //         tokens.push(trace_token_special!(", "));
            //     }
            // }
            // tokens.push(trace_token_special!("):"));
            // tokens
        }
    }
}
