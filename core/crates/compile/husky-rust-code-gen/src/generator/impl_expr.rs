use super::{impl_entity_route::EntityRouteRole, *};
use semantics_eager::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use vm::*;
use word::RootIdentifier;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => self.write(&varname),
            EagerExprVariant::ThisValue { .. } => self.write("self"),
            EagerExprVariant::ThisField { field_ident, .. } => match self.context {
                RustCodeGenContext::Normal => {
                    self.write("self.");
                    self.write(&field_ident.ident);
                }
                RustCodeGenContext::StructDerivedEager => {
                    self.write(&field_ident.ident);
                }
            },
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
                EagerOpnVariant::Binary { opr, .. } => {
                    match opr {
                        BinaryOpr::Pure(_) => (),
                        BinaryOpr::Assign(_) => match opds[0].variant {
                            EagerExprVariant::Variable { varname, binding } => (),
                            EagerExprVariant::Opn {
                                ref opn_variant, ..
                            } => match opn_variant {
                                EagerOpnVariant::Index { element_binding } => (),
                                _ => self.write("*"),
                            },
                            _ => self.write("*"),
                        },
                    }
                    self.gen_expr(&opds[0]);
                    self.write(opr.spaced_code());
                    self.gen_expr(&opds[1]);
                }
                EagerOpnVariant::Prefix { opr, .. } => match opr {
                    PrefixOpr::Not => match opds[0].ty() {
                        EntityRoutePtr::Root(RootIdentifier::Bool) => {
                            self.write("!");
                            self.gen_expr(&opds[0]);
                        }
                        _ => {
                            self.write("(0 == ");
                            self.gen_expr(&opds[0]);
                            self.write(")");
                        }
                    },
                    _ => {
                        self.write(&opr.rust_code());
                        self.gen_expr(&opds[0]);
                    }
                },
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.gen_expr(&opds[0]);
                    self.gen_suffix_opr(*opr)
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
                EagerOpnVariant::MethodCall {
                    method_ident,
                    method_route,
                    output_binding,
                    ..
                } => {
                    let method_decl = self.db.method_decl(*method_route).unwrap();
                    match method_decl.output.liason {
                        OutputLiason::Transfer => {
                            self.gen_expr(&opds[0]);
                            self.write(".");
                            self.write(&method_ident.ident);
                            self.write("(");
                            self.gen_arguments(&opds[1..]);
                            self.write(")");
                        }
                        OutputLiason::MemberAccess { .. } => match output_binding {
                            Binding::EvalRef | Binding::TempRef => {
                                self.gen_expr(&opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(&opds[1..]);
                                self.write(")");
                            }
                            Binding::Copy => {
                                self.write("*");
                                self.gen_expr(&opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(&opds[1..]);
                                self.write(")");
                            }
                            Binding::TempRefMut => {
                                self.gen_expr(&opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("_mut");
                                self.write("(");
                                self.gen_arguments(&opds[1..]);
                                self.write(")");
                            }
                            Binding::Move => todo!(),
                        },
                    }
                }
                EagerOpnVariant::Index { .. } => {
                    self.gen_expr(&opds[0]);
                    self.write("[");
                    if opds.len() > 2 {
                        todo!()
                    } else {
                        self.write("(");
                        self.gen_expr(&opds[1]);
                        self.write(")");
                        self.write(" as usize")
                    }
                    self.write("]");
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(value) => {
                self.gen_entity_route(value, EntityRouteRole::Other)
            }
        }
    }

    fn gen_arguments(&mut self, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_binding(expr);
            self.gen_expr(expr)
        }
    }

    pub(super) fn gen_binding(&mut self, expr: &EagerExpr) {
        match expr.qualified_ty.qual.binding(expr.contract) {
            Binding::EvalRef => (),
            Binding::TempRef => self.write("&"),
            Binding::TempRefMut => self.write("&mut "),
            Binding::Move => (),
            Binding::Copy => (),
        }
    }

    fn gen_copyable_literal(&mut self, v: CopyableValue) {
        match v {
            CopyableValue::I32(i) => {
                self.result.push_str(&i.to_string());
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

    fn gen_suffix_opr(&mut self, opr: SuffixOpr) {
        match opr {
            SuffixOpr::Incr => self.write(" += 1"),
            SuffixOpr::Decr => self.write(" -= 1"),
            SuffixOpr::AsTy(ty) => {
                self.write(" as ");
                self.gen_entity_route(ty.route, EntityRouteRole::Other)
            }
        }
    }
}
