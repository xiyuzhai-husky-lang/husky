mod impl_func_stmt;
mod impl_proc_stmt;

use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_word::Identifier;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureLazyStmt>,
    ) -> Vec<TraceId> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. }
            | FeatureLazyStmtVariant::Assert { .. }
            | FeatureLazyStmtVariant::Require { .. }
            | FeatureLazyStmtVariant::Return { .. } => {
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
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
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
                &self.runtime.comptime().text(entity.file).unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Proc {
                parameters: ref parameters,
                ..
            } => routine_call_head_tokens(
                &self.runtime.comptime().text(entity.file).unwrap(),
                "proc ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Module {
                ref module_items,
                ref opt_main_defn,
            } => todo!(),
            EntityDefnVariant::Feature { ref defn_repr } => todo!(),
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
            EntityDefnVariant::Method {
                spatial_parameters: ref generic_parameters,
                this_liason: this_contract,
                ref parameters,
                output_ty,
                output_liason,
                ..
            } => routine_call_head_tokens(
                &self.runtime.comptime().text(entity.file).unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Ty { .. } => todo!(),
            EntityDefnVariant::Trait { .. } => todo!(),
            EntityDefnVariant::EnumVariant {
                ref enum_variant_defn_variant,
            } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty: ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Input { .. } => todo!(),
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
            let mut tokens = vec![
                keyword!(routine_keyword),
                ident!(ident.as_str()),
                special!("("),
            ];
            for i in 0..parameters.len() {
                let input_placeholder = &parameters[i];
                tokens.push(ident!(input_placeholder.ranged_ident.ident.as_str()));
                tokens.push(special!(": "));
                tokens.push(route!(text.ranged(input_placeholder.ranged_ty.range)));
                if i < parameters.len() - 1 {
                    tokens.push(special!(", "));
                }
            }
            tokens.push(special!("):"));
            tokens
        }
    }
}
