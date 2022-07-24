use fold::Indent;
use husky_eager_semantics::{FuncStmt, ProcStmt};
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::DefinitionRepr;
use word::CustomIdentifier;

use super::{impl_entity_route::EntityRouteRole, *};

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_feature_defn(
        &mut self,
        feature_route: EntityRoutePtr,
        defn_repr: &DefinitionRepr,
    ) {
        match defn_repr {
            DefinitionRepr::LazyExpr { expr } => (),
            DefinitionRepr::LazyBlock { stmts, ty } => (),
            DefinitionRepr::FuncBlock {
                route,
                file,
                range,
                stmts,
                ty,
            } => self.gen_feature_func_block_defn(feature_route, ty.route, stmts),
            DefinitionRepr::ProcBlock {
                file,
                range,
                stmts,
                ty,
            } => todo!(),
        }
    }

    pub(super) fn gen_feature_func_block_defn(
        &mut self,
        feature_route: EntityRoutePtr,
        output: EntityRoutePtr,
        stmts: &[Arc<FuncStmt>],
    ) {
        self.write("pub(crate) fn ");
        let ident = feature_route.ident();
        self.write(&ident);
        self.write("<'eval>(__ctx: &dyn __EvalContext<'eval>) -> &'eval ");
        self.gen_entity_route(output.deref_route(), EntityRouteRole::Decl);
        self.write(&format!(
            r#" {{
    let __feature = feature_ptr!(__ctx, "{feature_route:?}");
    if let Some(__result) = __opt_cached_feature(__ctx, __feature) {{
        return __result.unwrap();
    }}
"#,
        ));
        self.gen_func_stmts(stmts);
        self.write("}\n");
    }

    pub(super) fn gen_proc_defn(
        &mut self,
        indent: Indent,
        base_route: EntityRoutePtr,
        parameters: &[Parameter],
        output: EntityRoutePtr,
        stmts: &[Arc<ProcStmt>],
    ) {
        let needs_eval_context: bool = self.db.needs_eval_context(base_route);
        let needs_eval_ref = (needs_eval_context
            || self.db.entity_route_kind_contains_eval_ref(base_route.kind))
            && !self
                .db
                .entity_route_kind_contains_eval_ref(base_route.parent().kind);
        self.write("\n");
        self.indent(indent);
        self.write("pub(crate) fn ");
        let ident = base_route.ident();
        self.write(&ident);
        if needs_eval_ref {
            self.write("<'eval>")
        }
        self.write("(");
        if needs_eval_context {
            self.write("__ctx: &dyn __EvalContext<'eval>");
        }
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 || needs_eval_context {
                self.write(", ");
            }
            self.write(&parameter.ranged_ident.ident);
            self.write(": ");
            match parameter.ranged_liason.liason {
                ParameterLiason::Pure => {
                    if !self.db.is_copyable(parameter.ranged_ty.route).unwrap() {
                        self.write("&")
                    }
                }
                ParameterLiason::EvalRef => self.write("&'eval "),
                ParameterLiason::Move => todo!(),
                ParameterLiason::TempRefMut => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::TempRef => todo!(),
            }
            self.gen_entity_route(parameter.ranged_ty.route, EntityRouteRole::Decl);
        }
        self.write(") -> ");
        self.gen_entity_route(output, EntityRouteRole::Decl);
        self.write(" {\n");
        self.gen_proc_stmts(stmts);
        self.indent(indent);
        self.write("}\n");
    }

    pub(super) fn gen_func_defn(
        &mut self,
        indent: Indent,
        base_route: EntityRoutePtr,
        parameters: &[Parameter],
        output: EntityRoutePtr,
        stmts: &[Arc<FuncStmt>],
    ) {
        let needs_eval_context: bool = self.db.needs_eval_context(base_route);
        let needs_eval_ref = needs_eval_context
            || self.db.entity_route_kind_contains_eval_ref(base_route.kind)
                && !self
                    .db
                    .entity_route_kind_contains_eval_ref(base_route.parent().kind);
        self.indent(indent);
        self.write("pub(crate) fn ");
        let ident = base_route.ident();
        self.write(&ident);
        if needs_eval_ref {
            self.write("<'eval>")
        }
        self.write("(");
        if needs_eval_context {
            self.write("__ctx: &dyn __EvalContext<'eval>");
        }
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 || needs_eval_context {
                self.write(", ");
            }
            self.gen_parameter(parameter)
        }
        self.write(") -> ");
        self.gen_entity_route(output, EntityRouteRole::Decl);
        self.write(" {\n");
        self.gen_func_stmts(stmts);
        self.indent(indent);
        self.write("}\n");
    }
}
