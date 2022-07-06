use husky_eager_semantics::{FuncStmt, ProcStmt};
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::CallFormSource;
use word::CustomIdentifier;

use super::{impl_entity_route::EntityRouteRole, *};

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_parameter(&mut self, parameter: &Parameter) {
        self.write(&parameter.ranged_ident.ident);
        self.write(": ");
        match parameter.ranged_liason.liason {
            ParameterLiason::Pure => {
                if !self.db.is_copyable(parameter.ranged_ty.route).unwrap() {
                    self.write("&")
                }
            }
            ParameterLiason::EvalRef => self.write("&'eval "),
            ParameterLiason::Move => (),
            ParameterLiason::TempRefMut => self.write("&mut "),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::TempRef => todo!(),
        }
        self.gen_entity_route(parameter.ranged_ty.route, EntityRouteRole::Decl);
    }

    pub(crate) fn gen_call_form_source(&mut self, source: &CallFormSource) {
        match source {
            CallFormSource::Func { stmts } => self.gen_func_stmts(stmts),
            CallFormSource::Proc { stmts } => self.gen_proc_stmts(stmts),
            CallFormSource::Lazy { stmts } => todo!(),
            CallFormSource::Static(_) => todo!(),
        }
    }
}
