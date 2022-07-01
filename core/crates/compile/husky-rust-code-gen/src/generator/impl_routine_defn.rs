use fold::Indent;
use husky_eager_semantics::{FuncStmt, ProcStmt};
use husky_entity_route_syntax::EntityRoutePtr;
use word::CustomIdentifier;

use super::{impl_entity_route::EntityRouteRole, *};

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_proc_defn(
        &mut self,
        indent: Indent,
        base_route: EntityRoutePtr,
        parameters: &[Parameter],
        output: EntityRoutePtr,
        stmts: &[Arc<ProcStmt>],
    ) {
        let needs_eval_ref = self.db.entity_route_kind_contains_eval_ref(base_route.kind)
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
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 {
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
        let needs_eval_ref = self.db.entity_route_kind_contains_eval_ref(base_route.kind)
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
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 {
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
