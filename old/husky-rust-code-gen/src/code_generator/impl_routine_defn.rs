use fold::Indent;
use husky_eager_semantics::{FuncStmt, ProcStmt};
use husky_ethereal_term::EtherealTerm;
use husky_item_semantics::DefinitionRepr;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_val_defn(&mut self, feature_route: EtherealTerm, defn_repr: &DefinitionRepr) {
        match defn_repr {
            DefinitionRepr::LazyExpr { .. } => (),
            DefinitionRepr::LazyBlock { .. } => (),
            DefinitionRepr::FuncBlock {
                stmts, return_ty, ..
            } => self.gen_feature_func_block_defn(feature_route, return_ty.route, stmts),
            DefinitionRepr::ProcBlock {
                stmts, return_ty, ..
            } => self.gen_feature_proc_block_defn(feature_route, return_ty.route, stmts),
        }
    }

    pub(super) fn gen_feature_func_block_defn(
        &mut self,
        feature_route: EtherealTerm,
        return_ty: EtherealTerm,
        stmts: &[Arc<FuncStmt>],
    ) {
        self.write("pub(crate) fn ");
        let ident = feature_route.ident();
        self.write(&ident);
        let is_output_option = return_ty.is_option();
        self.write(format!(
            "<'eval>(__ctx: &dyn __EvalContext<'eval>) -> {}&'eval ",
            match is_output_option {
                true => "Option<",
                false => "",
            },
        ));
        self.gen_item_route(return_ty.intrinsic(), EntityRouteRole::Decl);
        if is_output_option {
            self.write(">")
        }
        let mangled_return_ty_vtable = self.db.mangled_intrinsic_ty_vtable(return_ty);
        self.write(&format!(
            r#" {{
    let __feature = feature_ptr!(__ctx, "{feature_route:?}");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {{
        return __result
            .unwrap()
            .downcast_{}eval_ref(&__registration__::{mangled_return_ty_vtable});
    }}
"#,
            match return_ty.is_option() {
                true => "opt_",
                false => "",
            }
        ));
        self.gen_func_stmts(stmts);
        self.write("}\n");
    }

    pub(super) fn gen_feature_proc_block_defn(
        &mut self,
        feature_route: EtherealTerm,
        return_ty: EtherealTerm,
        stmts: &[Arc<ProcStmt>],
    ) {
        self.write("pub(crate) fn ");
        let ident = feature_route.ident();
        self.write(&ident);
        let is_output_option = return_ty.is_option();
        self.write(format!(
            "<'eval>(__ctx: &dyn __EvalContext<'eval>) -> {}&'eval ",
            match is_output_option {
                true => "Option<",
                false => "",
            },
        ));
        self.gen_item_route(return_ty.intrinsic(), EntityRouteRole::Decl);
        if is_output_option {
            self.write(">")
        }
        let mangled_return_ty_vtable = self.db.mangled_intrinsic_ty_vtable(return_ty);
        self.write(&format!(
            r#" {{
    let __feature = feature_ptr!(__ctx, "{feature_route:?}");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {{
        return __result
            .unwrap()
            .downcast_{}eval_ref(&__registration__::{mangled_return_ty_vtable});
    }}
"#,
            match return_ty.is_option() {
                true => "opt_",
                false => "",
            }
        ));
        self.gen_proc_stmts(stmts);
        self.write("}\n");
    }

    pub(super) fn gen_proc_defn(
        &mut self,
        _indent: Indent,
        _base_route: EtherealTerm,
        _parameters: &[Parameter],
        _output: EtherealTerm,
        _stmts: &[Arc<ProcStmt>],
    ) {
        todo!()
        // let needs_eval_context: bool = self.db.needs_eval_context(base_route);
        // let needs_eval_ref = (needs_eval_context
        //     || self.db.item_route_variant_contains_eval_ref(base_route))
        //     && !self
        //         .db
        //         .item_route_variant_contains_eval_ref(base_route.parent());
        // self.write("\n");
        // self.indent(indent);
        // self.write("pub(crate) fn ");
        // let ident = base_route.ident();
        // self.write(&ident);
        // if needs_eval_ref {
        //     self.write("<'eval>")
        // }
        // self.write("(");
        // for (i, parameter) in parameters.iter().enumerate() {
        //     if i > 0 {
        //         self.write(", ");
        //     }
        //     self.write(&parameter.ident());
        //     self.write(": ");
        //     match parameter.contract() {
        //         ParameterModifier::None => {
        //             if !self.db.is_copyable(parameter.ty()).unwrap() {
        //                 self.write("&")
        //             }
        //         }
        //         ParameterModifier::Leash => self.write("&'eval "),
        //         ParameterModifier::Owned => todo!(),
        //         ParameterModifier::TempRefMut => todo!(),
        //         ParameterModifier::OwnedMut => todo!(),
        //         ParameterModifier::MemberAccess => todo!(),
        //         ParameterModifier::TempRef => todo!(),
        //     }
        //     self.gen_item_route(parameter.ty(), EntityRouteRole::Decl);
        // }
        // msg_once!("todo: keyword arguments, variadics");
        // if needs_eval_context {
        //     if parameters.len() > 0 {
        //         self.write(", ")
        //     }
        //     self.write("__ctx: &dyn __EvalContext<'eval>");
        // }
        // self.write(") -> ");
        // self.gen_item_route(output, EntityRouteRole::Decl);
        // self.write(" {\n");
        // self.gen_proc_stmts(stmts);
        // self.indent(indent);
        // self.write("}\n");
    }

    pub(super) fn gen_func_defn(
        &mut self,
        indent: Indent,
        base_route: EtherealTerm,
        parameters: &[Parameter],
        output: EtherealTerm,
        stmts: &[Arc<FuncStmt>],
    ) {
        let needs_eval_context: bool = self.db.needs_eval_context(base_route);
        let needs_eval_ref = needs_eval_context
            || self.db.item_route_variant_contains_eval_ref(base_route)
                && !self
                    .db
                    .item_route_variant_contains_eval_ref(base_route.parent());
        self.indent(indent);
        self.write("pub(crate) fn ");
        let ident = base_route.ident();
        self.write(&ident);
        if needs_eval_ref {
            self.write("<'eval>")
        }
        self.write("(");
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_parameter(parameter)
        }
        msg_once!("keyword arguments, variadics");
        if needs_eval_context {
            if parameters.len() > 0 {
                self.write(", ")
            }
            self.write("__ctx: &dyn __EvalContext<'eval>");
        }
        self.write(") -> ");
        self.gen_item_route(output, EntityRouteRole::Decl);
        self.write(" {\n");
        self.gen_func_stmts(stmts);
        self.indent(indent);
        self.write("}\n");
    }
}
