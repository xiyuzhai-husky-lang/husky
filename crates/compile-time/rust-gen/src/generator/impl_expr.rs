use super::*;
use semantics_eager::{EagerExpr, EagerExprKind, EagerOpnKind};
use syntax_types::SuffixOpr;
use vm::PrimitiveValue;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_expr(&mut self, expr: &EagerExpr) {
        match expr.kind {
            EagerExprKind::Variable(varname) => self.write(&varname),
            EagerExprKind::This => self.write("self"),
            EagerExprKind::Scope { scope, compiled } => todo!(),
            EagerExprKind::PrimitiveLiteral(value) => self.gen_primitive_literal(value),
            EagerExprKind::Bracketed(_) => todo!(),
            EagerExprKind::Opn {
                ref opn_kind,
                compiled,
                ref opds,
            } => match opn_kind {
                EagerOpnKind::Binary { opr, this } => {
                    self.gen_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.gen_expr(&opds[1]);
                }
                EagerOpnKind::Prefix { opr, this } => todo!(),
                EagerOpnKind::Suffix { opr, this } => match opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(memb_ident) => {
                        self.gen_expr(&opds[0]);
                        self.write(".");
                        self.write(&memb_ident)
                    }
                    SuffixOpr::WithType(_) => todo!(),
                },
                EagerOpnKind::RoutineCall(_) => todo!(),
                EagerOpnKind::TypeCall { ranged_ty, ty_decl } => {
                    self.gen_scope(ranged_ty.scope);
                    self.write("::");
                    self.write("__call__(");
                    self.gen_arguments(opds);
                    self.write(")");
                }
                EagerOpnKind::PatternCall => todo!(),
                EagerOpnKind::MembVarAccess { memb_var_contract } => todo!(),
                EagerOpnKind::MembRoutineCall { memb_ident, .. } => {
                    self.gen_expr(&opds[0]);
                    self.write(".");
                    self.write(&memb_ident);
                    self.write("(");
                    self.gen_arguments(&opds[1..]);
                    self.write(")");
                }
                EagerOpnKind::ElementAccess => todo!(),
            },
            EagerExprKind::Lambda(_, _) => todo!(),
        }
    }

    fn gen_arguments(&mut self, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_expr(expr)
        }
    }

    fn gen_primitive_literal(&mut self, v: PrimitiveValue) {
        match v {
            PrimitiveValue::I32(i) => {
                self.result.push_str(&i.to_string());
                self.write("i32")
            }
            PrimitiveValue::F32(_) => todo!(),
            PrimitiveValue::B32(_) => todo!(),
            PrimitiveValue::B64(_) => todo!(),
            PrimitiveValue::Bool(_) => todo!(),
            PrimitiveValue::Void => todo!(),
        }
    }
}
