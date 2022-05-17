use super::*;
use semantics_eager::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use vm::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable(varname) => self.write(&varname),
            EagerExprVariant::This => self.write("self"),
            EagerExprVariant::EntityRoute { route: scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => self.gen_primitive_literal(value),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    self.gen_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.gen_expr(&opds[1]);
                }
                EagerOpnVariant::Prefix { opr, this_ty: this } => todo!(),
                EagerOpnVariant::Suffix { opr, this } => match opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::FieldAccess(field_ident) => {
                        self.gen_expr(&opds[0]);
                        self.write(".");
                        self.write(&field_ident.ident)
                    }
                    SuffixOpr::WithTy(_) => todo!(),
                    SuffixOpr::AsTy(_) => todo!(),
                },
                EagerOpnVariant::RoutineCall(_) => todo!(),
                EagerOpnVariant::TypeCall { ranged_ty, ty_decl } => {
                    self.gen_entity_route(ranged_ty.route);
                    self.write("::");
                    self.write("__call__(");
                    self.gen_arguments(opds);
                    self.write(")");
                }
                EagerOpnVariant::PatternCall => todo!(),
                EagerOpnVariant::FieldAccess {
                    field_contract: field_var_contract,
                } => todo!(),
                EagerOpnVariant::MethodCall {
                    method_ident: field_ident,
                    ..
                } => {
                    self.gen_expr(&opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident);
                    self.write("(");
                    self.gen_arguments(&opds[1..]);
                    self.write(")");
                }
                EagerOpnVariant::ElementAccess => {
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
            PrimitiveValue::F32(f) => {
                self.result.push_str(&f.to_string());
                self.write("f32")
            }
            PrimitiveValue::B32(b) => {
                self.result.push_str(&b.to_string());
                self.write("u32")
            }
            PrimitiveValue::B64(b) => {
                self.result.push_str(&b.to_string());
                self.write("u64")
            }
            PrimitiveValue::Bool(b) => self.result.push_str(&b.to_string()),
            PrimitiveValue::Void => self.result.push_str("()"),
        }
    }
}
