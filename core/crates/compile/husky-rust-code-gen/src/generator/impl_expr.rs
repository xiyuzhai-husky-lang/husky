use super::{impl_entity_route::EntityRouteRole, *};
use semantics_eager::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use vm::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => self.write(&varname),
            EagerExprVariant::ThisValue { .. } => self.write("self"),
            EagerExprVariant::ThisField { field_ident, .. } => {
                self.write("self.");
                self.write(&field_ident.ident);
            }
            EagerExprVariant::EntityRoute { route } => self.write(&format!("{:?}", route)),
            EagerExprVariant::PrimitiveLiteral(value) => self.gen_copyable_literal(value),
            EagerExprVariant::Bracketed(ref expr) => {
                self.write("(");
                self.gen_expr(expr);
                self.write(")")
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    self.gen_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.gen_expr(&opds[1]);
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    self.write(&opr.rust_code());
                    self.gen_expr(&opds[0]);
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.gen_expr(&opds[0]);
                    self.write(&opr.rust_code());
                }
                EagerOpnVariant::RoutineCall(routine) => {
                    self.gen_entity_route(routine.route, EntityRouteRole::Caller);
                    self.write("(");
                    self.gen_arguments(opds);
                    self.write(")");
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    self.gen_entity_route(ranged_ty.route, EntityRouteRole::Caller);
                    self.write("::");
                    self.write("__call__(");
                    self.gen_arguments(opds);
                    self.write(")");
                }
                EagerOpnVariant::FieldAccess { field_ident, .. } => {
                    self.gen_expr(&opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident)
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    self.gen_expr(&opds[0]);
                    self.write(".");
                    self.write(&method_ident.ident);
                    self.write("(");
                    self.gen_arguments(&opds[1..]);
                    self.write(")");
                }
                EagerOpnVariant::ElementAccess { .. } => {
                    self.gen_expr(&opds[0]);
                    self.write("[");
                    if opds.len() > 2 {
                        todo!()
                    } else {
                        self.gen_expr(&opds[1])
                    }
                    self.write("]");
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(value) => {
                self.gen_entity_route(value, EntityRouteRole::Value)
            }
        }
    }

    fn gen_arguments(&mut self, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            match expr.qualified_ty.qual.binding(expr.contract) {
                Binding::EvalRef => (),
                Binding::TempRef => self.write("&"),
                Binding::TempRefMut => self.write("&mut "),
                Binding::Move => (),
                Binding::Copy => (),
            }
            self.gen_expr(expr)
        }
    }

    fn gen_copyable_literal(&mut self, v: CopyableValue) {
        match v {
            CopyableValue::I32(i) => {
                self.result.push_str(&i.to_string());
                self.write("i32")
            }
            CopyableValue::F32(f) => {
                self.result.push_str(&f.to_string());
                self.write("f32")
            }
            CopyableValue::B32(b) => {
                self.result.push_str(&b.to_string());
                self.write("u32")
            }
            CopyableValue::B64(b) => {
                self.result.push_str(&b.to_string());
                self.write("u64")
            }
            CopyableValue::Bool(b) => self.result.push_str(&b.to_string()),
            CopyableValue::Void(_) => self.result.push_str("()"),
            CopyableValue::EnumKind(_) => todo!(),
        }
    }
}
