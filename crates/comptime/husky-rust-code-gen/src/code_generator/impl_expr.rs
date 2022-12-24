use super::*;
use fold::Indent;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use husky_entity_taxonomy::FieldKind;

use husky_opn_semantics::{EagerSuffixOpr, ImplicitConversion};
use husky_primitive_literal_syntax::LiteralToken;

use husky_word::RootBuiltinIdentifier;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_expr(&mut self, indent: Indent, expr: &EagerExpr) {
        match expr.implicit_conversion {
            ImplicitConversion::None => (),
            ImplicitConversion::WrapInSome { number_of_somes } => {
                for _ in 0..number_of_somes {
                    self.write("Some(")
                }
            }
            ImplicitConversion::ConvertToBool => todo!(),
        }

        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => self.write(&varname),
            EagerExprVariant::ThisValue { .. } => self.write("self"),
            EagerExprVariant::ThisField { field_ident, .. } => match self.context {
                RustCodeGenContext::Normal => {
                    self.write("self.");
                    self.write(&field_ident.ident);
                }
                RustCodeGenContext::ThisFieldWithPrefix { prefix } => {
                    self.write(prefix);
                    self.write(&field_ident.ident);
                }
            },
            EagerExprVariant::PrimitiveLiteral(value) => self.gen_primitive_literal(value),
            EagerExprVariant::Bracketed(ref expr) => {
                self.write("(");
                self.gen_expr(indent, expr);
                self.write(")")
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, .. } => {
                    match opr {
                        BinaryOpr::PureClosed(_) => (),
                        BinaryOpr::Assign(_) => match opds[0].variant {
                            EagerExprVariant::Variable { .. } => (),
                            EagerExprVariant::Opn {
                                ref opn_variant, ..
                            } => match opn_variant {
                                EagerOpnVariant::Index { .. } => (),
                                EagerOpnVariant::Field { .. } => (),
                                _ => self.write("*"),
                            },
                            _ => self.write("*"),
                        },
                        BinaryOpr::ScopeResolution => todo!(),
                        BinaryOpr::Curry => todo!(),
                        BinaryOpr::As => todo!(),
                        BinaryOpr::Comparison(_) => todo!(),
                        BinaryOpr::ShortcuitLogic(_) => todo!(),
                    }
                    self.gen_expr(indent, &opds[0]);
                    match opr {
                        BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid) => {
                            self.write(".rem_euclid(");
                            self.gen_expr(indent, &opds[1]);
                            self.write(")")
                        }
                        BinaryOpr::Assign(Some(BinaryPureClosedOpr::RemEuclid)) => todo!(),
                        _ => {
                            self.write(opr.spaced_code());
                            self.gen_expr(indent, &opds[1]);
                        }
                    }
                }
                EagerOpnVariant::Prefix { opr, .. } => match opr {
                    PrefixOpr::Not => match opds[0].intrinsic_ty() {
                        Term::Root(RootBuiltinIdentifier::Bool) => {
                            self.write("!");
                            self.gen_expr(indent, &opds[0]);
                        }
                        _ => {
                            self.write("(0 == ");
                            self.gen_expr(indent, &opds[0]);
                            self.write(")");
                        }
                    },
                    _ => {
                        self.write(&opr.rust_code());
                        self.gen_expr(indent, &opds[0]);
                    }
                },
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.gen_suffix_opr(opr)
                }
                // EagerOpnVariant::RoutineCall(_routine) => {
                //     todo!()
                //     // self.gen_entity_route(routine.route, EntityRouteRole::Caller);
                //     // self.write("(");
                //     // self.gen_arguments(indent, opds);
                //     // if self.db.needs_eval_context(routine.route) {
                //     //     if opds.len() > 0 {
                //     //         self.write(", ")
                //     //     }
                //     //     self.write("__ctx")
                //     // }
                //     // self.write(")");
                // }
                EagerOpnVariant::TypeCall { .. } => {
                    todo!()
                    // self.gen_type_call_opn(indent, ranged_ty.route, opds, ty_decl);
                }
                EagerOpnVariant::Field {
                    field_ident,
                    field_kind,
                    ..
                } => {
                    self.gen_expr(indent, &opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident);
                    match field_kind {
                        FieldKind::StructRegular
                        | FieldKind::StructDefault
                        | FieldKind::StructDerived => (),
                        FieldKind::StructMemo => self.write("(__ctx)"),
                        FieldKind::RecordRegular => todo!(),
                        FieldKind::RecordProperty => todo!(),
                    }
                }
                EagerOpnVariant::MethodCall { .. } => {
                    todo!()
                    // let call_form_decl = self.db.entity_call_form_decl(*method_route).unwrap();
                    // match call_form_decl.output.liason() {
                    //     OutputModifier::Transfer => {
                    //         self.gen_expr(indent, &opds[0]);
                    //         self.write(".");
                    //         self.write(&method_ident.ident);
                    //         // ad hoc
                    //         if method_ident.ident.as_str() == "pop_with_largest_opt_f32" {
                    //             let elem_ty = method_route.parent().entity_route_argument(0);
                    //             if self.db.is_copyable(elem_ty).unwrap() {
                    //                 self.write("_copyable")
                    //             } else {
                    //                 self.write("_borrow")
                    //             }
                    //         }
                    //         self.write("(");
                    //         self.gen_arguments(indent, &opds[1..]);
                    //         if self.db.needs_eval_context(*method_route) {
                    //             if opds.len() > 1 {
                    //                 self.write(", ")
                    //             }
                    //             self.write("__ctx")
                    //         }
                    //         self.write(")");
                    //     }
                    //     OutputModifier::MemberAccess { .. } => match output_binding {
                    //         Binding::EvalRef | Binding::TempRef => {
                    //             self.gen_expr(indent, &opds[0]);
                    //             self.write(".");
                    //             self.write(&method_ident.ident);
                    //             self.write("(");
                    //             self.gen_arguments(indent, &opds[1..]);
                    //             self.write(")");
                    //         }
                    //         Binding::Copy => {
                    //             self.write("*");
                    //             self.gen_expr(indent, &opds[0]);
                    //             self.write(".");
                    //             self.write(&method_ident.ident);
                    //             self.write("(");
                    //             self.gen_arguments(indent, &opds[1..]);
                    //             self.write(")");
                    //         }
                    //         Binding::TempMut => {
                    //             self.gen_expr(indent, &opds[0]);
                    //             self.write(".");
                    //             self.write(&method_ident.ident);
                    //             self.write("_mut");
                    //             self.write("(");
                    //             self.gen_arguments(indent, &opds[1..]);
                    //             self.write(")");
                    //         }
                    //         Binding::Move => todo!(),
                    //         Binding::DerefCopy => todo!(),
                    //     },
                    // }
                }
                EagerOpnVariant::Index { .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.write("[");
                    if opds.len() > 2 {
                        todo!()
                    } else {
                        self.write("(");
                        self.gen_expr(indent, &opds[1]);
                        self.write(")");
                        self.write(" as usize")
                    }
                    self.write("]");
                }
                EagerOpnVariant::NewVecFromList => {
                    self.write("vec![");
                    for (i, opd) in opds.iter().enumerate() {
                        if i > 0 {
                            self.write(", ")
                        }
                        self.gen_expr(indent, opd)
                    }
                    self.write("]")
                }
                EagerOpnVariant::ValueCall => {
                    for (i, opd) in opds.iter().enumerate() {
                        if i > 1 {
                            self.write(", ")
                        }
                        self.gen_expr(indent, opd);
                        if i == 0 {
                            self.write(format!(".call{}(", opds.len() - 1))
                        }
                    }
                    self.write(", __ctx)")
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(value) => {
                self.gen_entity_route(value, EntityRouteRole::Other)
            }
            EagerExprVariant::EntityFeature { .. } => {
                todo!()
                // self.gen_entity_route(route, EntityRouteRole::Caller);
                // self.write("(__ctx)")
            }
            EagerExprVariant::EntityThickFp { .. } => {
                todo!()
                // let ty = expr.intrinsic_ty();
                // self.write("ThickFp::");
                // // is self.db.needs_eval_context(ty) necessary?
                // let needs_eval_context =
                //     self.db.needs_eval_context(ty) || self.db.needs_eval_context(route);
                // if needs_eval_context {
                //     self.write("__new_ctx(")
                // } else {
                //     self.write("__new_base(")
                // }
                // self.gen_entity_route(route, EntityRouteRole::FpValue);
                // self.write(" as ");
                // self.gen_entity_route(
                //     ty,
                //     EntityRouteRole::StaticThinFpTyDecl { needs_eval_context },
                // );
                // self.write(")")
            }
        }

        match expr.implicit_conversion {
            ImplicitConversion::None => (),
            ImplicitConversion::WrapInSome { number_of_somes } => {
                for _ in 0..number_of_somes {
                    self.write(")")
                }
            }
            ImplicitConversion::ConvertToBool => todo!(),
        }
    }
    // fn gen_type_call_opn(
    //     &mut self,
    //     indent: Indent,
    //     ty: EntityRoutePtr,
    //     opds: &Vec<Arc<EagerExpr>>,
    //     ty_decl: &Arc<infer_decl::TyDecl>,
    // ) {
    //     let type_call = ty_decl.opt_type_call.as_ref().unwrap();
    //     self.exec_within_context(
    //         RustCodeGenContext::ThisFieldWithPrefix { prefix: "self." },
    //         |this| {
    //             if type_call.keyword_parameters.len() > 0 {
    //                 this.gen_type_call_opn_with_keyword_parameters(indent, ty, opds, type_call)
    //             } else {
    //                 this.gen_type_call_opn_without_keyword_parameters(indent, ty, opds, type_call)
    //             }
    //         },
    //     )
    // }

    // fn gen_type_call_opn_without_keyword_parameters(
    //     &mut self,
    //     indent: Indent,
    //     ty: EntityRoutePtr,
    //     opds: &Vec<Arc<EagerExpr>>,
    //     type_call: &CallFormDecl,
    // ) {
    //     self.gen_entity_route(ty, EntityRouteRole::Caller);
    //     self.write("::");
    //     self.write("__call__(");
    //     self.gen_arguments(indent, opds);
    //     msg_once!("variadics");
    //     match type_call.variadic_parameters {
    //         VariadicParametersDecl::None => (),
    //         VariadicParametersDecl::RepeatSingle { .. } => {
    //             if type_call.primary_parameters.len() + type_call.keyword_parameters.len() > 0 {
    //                 self.write(", ")
    //             }
    //             self.write("vec![]")
    //         }
    //     }
    //     self.write(")");
    // }

    // fn gen_type_call_opn_with_keyword_parameters(
    //     &mut self,
    //     indent: Indent,
    //     ty: EntityRoutePtr,
    //     opds: &Vec<Arc<EagerExpr>>,
    //     type_call: &CallFormDecl,
    // ) {
    //     self.write("{\n");
    //     self.indent(indent + 8);
    //     let ty_defn = self.db.entity_defn(ty).unwrap();
    //     let ty_members = match ty_defn.variant {
    //         EntityDefnVariant::Term { ref ty_members, .. } => ty_members,
    //         _ => panic!(),
    //     };
    //     for (_i, (parameter, expr)) in
    //         std::iter::zip(type_call.primary_parameters.iter(), opds.iter()).enumerate()
    //     {
    //         self.write("let __this_");
    //         self.write(&parameter.ident);
    //         self.write(": ");
    //         self.gen_entity_route(parameter.ty(), EntityRouteRole::Decl);
    //         self.write(" = ");
    //         self.gen_binding(expr);
    //         self.gen_expr(indent, expr);
    //         self.write(";");
    //         self.newline_indented(indent + 8);
    //     }
    //     for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
    //         self.write("let __this_");
    //         self.write(&parameter.ident);
    //         match ty_members.data()[type_call.primary_parameters.len() + i].variant {
    //             EntityDefnVariant::TyField {
    //                 ref field_variant, ..
    //             } => match field_variant {
    //                 FieldDefnVariant::StructDefault { default } => {
    //                     self.write(": ");
    //                     self.gen_entity_route(parameter.ty(), EntityRouteRole::Decl);
    //                     self.write(" = ");
    //                     self.exec_within_context(
    //                         RustCodeGenContext::ThisFieldWithPrefix { prefix: "__this_" },
    //                         |this| this.gen_expr(indent + 4, default),
    //                     );
    //                     self.write(";");
    //                 }
    //                 _ => panic!(),
    //             },
    //             _ => panic!(),
    //         }
    //         self.newline_indented(indent + 8);
    //     }
    //     self.gen_entity_route(ty, EntityRouteRole::Caller);
    //     self.write("::");
    //     self.write("__call__(");
    //     for (i, parameter) in type_call.primary_parameters.iter().enumerate() {
    //         if i > 0 {
    //             self.write(", ")
    //         }
    //         self.write("__this_");
    //         self.write(&parameter.ident)
    //     }
    //     for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
    //         if i + type_call.primary_parameters.len() > 0 {
    //             self.write(", ")
    //         }
    //         self.write("__this_");
    //         self.write(&parameter.ident)
    //     }
    //     msg_once!("keyword arguments and more on variadics");
    //     match type_call.variadic_parameters {
    //         VariadicParametersDecl::None => (),
    //         VariadicParametersDecl::RepeatSingle { .. } => {
    //             if type_call.primary_parameters.len() + type_call.keyword_parameters.len() > 0 {
    //                 self.write(", ")
    //             }
    //             self.write("vec![]")
    //         }
    //     }
    //     self.write(")");
    //     self.newline_indented(indent + 4);
    //     self.write("}");
    // }

    // pub(super) fn gen_feature_return(
    //     &mut self,
    //     indent: Indent,
    //     result: &EagerExpr,
    //     output_ty: EntityRoutePtr,
    // ) {
    //     let mangled_intrinsic_ty_vtable =
    //         self.db.mangled_intrinsic_ty_vtable(result.intrinsic_ty());
    //     match result.qualified_ty.qual() {
    //         EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
    //             self.write(
    //                 r#"__ctx
    //     .cache_feature(
    //         __feature,
    //         Ok(__Register::new_box::<"#,
    //             );
    //             self.gen_entity_route(output_ty.intrinsic(), EntityRouteRole::Decl);
    //             self.write(">(");
    //             self.gen_expr(indent, result);
    //             self.write(&format!(
    //                 r#", &__registration__::{mangled_intrinsic_ty_vtable})),
    //     )
    //     .unwrap()
    //     .downcast_{}eval_ref(&__registration__::{mangled_intrinsic_ty_vtable})"#,
    //                 match output_ty.is_option() {
    //                     true => "opt_",
    //                     false => "",
    //                 }
    //             ));
    //         }
    //         EagerExprQualifier::EvalRef => {
    //             self.write(
    //                 r#"__ctx
    //     .cache_feature(
    //         __feature,
    //         Ok(__Register::new_eval_ref::<"#,
    //             );
    //             self.gen_entity_route(output_ty.intrinsic(), EntityRouteRole::Decl);
    //             self.write(r#">(&("#);
    //             self.gen_expr(indent, result);
    //             self.write(&format!(
    //                 r#"), &__registration__::{mangled_intrinsic_ty_vtable}).into()),
    //     )
    //     .unwrap().downcast_{}eval_ref(&__registration__::{mangled_intrinsic_ty_vtable})"#,
    //                 match output_ty.is_option() {
    //                     true => "opt_",
    //                     false => "",
    //                 }
    //             ));
    //         }
    //         EagerExprQualifier::PureRef
    //         | EagerExprQualifier::TempRef
    //         | EagerExprQualifier::TempRefMut => panic!(),
    //     }
    // }

    // pub(super) fn gen_lazy_field_return(
    //     &mut self,
    //     indent: Indent,
    //     result: &EagerExpr,
    //     output_ty: EntityRoutePtr,
    // ) {
    //     let mangled_intrinsic_ty_vtable =
    //         self.db.mangled_intrinsic_ty_vtable(result.intrinsic_ty());
    //     match result.qualified_ty.qual() {
    //         EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
    //             self.write(
    //                 r#"__ctx.cache_lazy_field(
    //     self as *const _ as *const std::ffi::c_void,
    //     __uid,
    //     Ok(__Register::new_box::<"#,
    //             );
    //             self.gen_entity_route(output_ty.intrinsic(), EntityRouteRole::Decl);
    //             self.write(r#">("#);
    //             self.gen_expr(indent, result);
    //             self.write(format!(
    //                 r#", &__registration__::{mangled_intrinsic_ty_vtable}))
    // ).unwrap().downcast_{}eval_ref(&__registration__::{mangled_intrinsic_ty_vtable})"#,
    //                 match output_ty.is_option() {
    //                     true => "opt_",
    //                     false => "",
    //                 }
    //             ));
    //         }
    //         EagerExprQualifier::EvalRef => {
    //             self.write(format!(
    //                 r#"__ctx.cache_lazy_field(
    //     self as *const _ as *const std::ffi::c_void,
    //     __uid,
    //     Ok(__Register::new_{}eval_ref::<"#,
    //                 match output_ty.is_option() {
    //                     true => "opt_",
    //                     false => "",
    //                 }
    //             ));
    //             self.gen_entity_route(output_ty.intrinsic(), EntityRouteRole::Decl);
    //             self.write(r#">(&("#);
    //             self.gen_expr(indent, result);
    //             self.write(format!(
    //                 r#"), &__registration__::{mangled_intrinsic_ty_vtable})).into()
    // ).unwrap().downcast_{}eval_ref(&__registration__::{mangled_intrinsic_ty_vtable})"#,
    //                 match output_ty.is_option() {
    //                     true => "opt_",
    //                     false => "",
    //                 }
    //             ));
    //         }
    //         EagerExprQualifier::PureRef
    //         | EagerExprQualifier::TempRef
    //         | EagerExprQualifier::TempRefMut => panic!(),
    //     }
    // }

    // fn gen_arguments(&mut self, indent: Indent, exprs: &[Arc<EagerExpr>]) {
    //     for (i, expr) in exprs.iter().enumerate() {
    //         if i > 0 {
    //             self.write(", ");
    //         }
    //         self.gen_binding(expr);
    //         self.gen_expr(indent, expr)
    //     }
    // }

    // pub(super) fn gen_binding(&mut self, expr: &EagerExpr) {
    //     match expr.qualified_ty.binding(self.db.upcast(), expr.contract) {
    //         Binding::EvalRef | Binding::TempRef => {
    //             if expr.qualified_ty.option_level() == 0 {
    //                 self.write("&")
    //             }
    //         }
    //         Binding::TempMut => self.write("&mut "),
    //         Binding::Move => (),
    //         Binding::Copy => (),
    //         Binding::DerefCopy => self.write("*"),
    //     }
    // }

    fn gen_primitive_literal(&mut self, v: LiteralToken) {
        match v {
            LiteralToken::Integer(i) => {
                self.result.push_str(&i.to_string());
            }
            LiteralToken::I32(i) => {
                self.result.push_str(&i.to_string());
                self.write("i32")
            }
            LiteralToken::I64(i) => {
                self.result.push_str(&i.to_string());
                self.write("i64")
            }
            LiteralToken::Float(f) => {
                self.result.push_str(&f.to_string());
                msg_once!("ad hoc");
                self.write("f32")
            }
            LiteralToken::F32(f) => {
                self.result.push_str(&f.to_string());
                self.write("f32")
            }
            LiteralToken::F64(f) => {
                self.result.push_str(&f.to_string());
                self.write("f64")
            }
            LiteralToken::Bits(b) => self.result.push_str(&b.to_string()),
            LiteralToken::B32(b) => {
                self.result.push_str(&b.to_string());
                self.write("u32")
            }
            LiteralToken::B64(b) => {
                self.result.push_str(&b.to_string());
                self.write("u64")
            }
            LiteralToken::Bool(b) => self.result.push_str(&b.to_string()),
            LiteralToken::Unit => self.result.push_str("()"),
        }
    }

    fn gen_suffix_opr(&mut self, opr: &EagerSuffixOpr) {
        match opr {
            EagerSuffixOpr::Incr => self.write(" += 1"),
            EagerSuffixOpr::Decr => self.write(" -= 1"),
            EagerSuffixOpr::AsTy(ty) => {
                self.write(" as ");
                self.gen_entity_route(ty.route, EntityRouteRole::Other)
            }
            EagerSuffixOpr::BePattern(_) => todo!(),
            EagerSuffixOpr::Unveil => panic!("shouldn't be handled here"),
        }
    }
}
