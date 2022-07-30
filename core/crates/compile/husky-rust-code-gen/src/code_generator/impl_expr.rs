use super::{impl_entity_route::EntityRouteRole, *};
use fold::Indent;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use husky_entity_semantics::FieldDefnVariant;
use husky_infer_qualified_ty::EagerExprQualifier;
use husky_opn_semantics::SuffixOpr;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_vm_binding::Binding;
use husky_word::RootIdentifier;
use infer_decl::{CallFormDecl, VariadicTemplate};

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_expr(&mut self, indent: Indent, expr: &EagerExpr) {
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
                        BinaryOpr::Pure(_) => (),
                        BinaryOpr::Assign(_) => match opds[0].variant {
                            EagerExprVariant::Variable { varname, binding } => (),
                            EagerExprVariant::Opn {
                                ref opn_variant, ..
                            } => match opn_variant {
                                EagerOpnVariant::Index { .. } => (),
                                EagerOpnVariant::Field { .. } => (),
                                _ => self.write("*"),
                            },
                            _ => self.write("*"),
                        },
                    }
                    self.gen_expr(indent, &opds[0]);
                    match opr {
                        BinaryOpr::Pure(PureBinaryOpr::RemEuclid) => {
                            self.write(".rem_euclid(");
                            self.gen_expr(indent, &opds[1]);
                            self.write(")")
                        }
                        BinaryOpr::Assign(Some(PureBinaryOpr::RemEuclid)) => todo!(),
                        _ => {
                            self.write(opr.spaced_code());
                            self.gen_expr(indent, &opds[1]);
                        }
                    }
                }
                EagerOpnVariant::Prefix { opr, .. } => match opr {
                    PrefixOpr::Not => match opds[0].ty() {
                        EntityRoutePtr::Root(RootIdentifier::Bool) => {
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
                EagerOpnVariant::RoutineCall(routine) => {
                    self.gen_entity_route(routine.route, EntityRouteRole::Caller);
                    self.write("(");
                    self.gen_arguments(indent, opds);
                    self.write(")");
                }
                EagerOpnVariant::TypeCall {
                    ranged_ty,
                    ref ty_decl,
                    ..
                } => {
                    self.gen_type_call_opn(indent, ranged_ty.route, opds, ty_decl);
                }
                EagerOpnVariant::Field { field_ident, .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident)
                }
                EagerOpnVariant::MethodCall {
                    method_ident,
                    method_route,
                    output_binding,
                    ..
                } => {
                    let call_form_decl = self.db.entity_call_form_decl(*method_route).unwrap();
                    match call_form_decl.output.liason {
                        OutputLiason::Transfer => {
                            self.gen_expr(indent, &opds[0]);
                            self.write(".");
                            self.write(&method_ident.ident);
                            self.write("(");
                            self.gen_arguments(indent, &opds[1..]);
                            self.write(")");
                        }
                        OutputLiason::MemberAccess { .. } => match output_binding {
                            Binding::EvalRef | Binding::TempRef => {
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::Copy => {
                                self.write("*");
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::TempMut => {
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("_mut");
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::Move => todo!(),
                        },
                    }
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
                            self.write("(")
                        }
                    }
                    self.write(")")
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(value) => {
                self.gen_entity_route(value, EntityRouteRole::Other)
            }
            EagerExprVariant::EntityFeature { route } => {
                self.gen_entity_route(route, EntityRouteRole::Caller);
                self.write("(__ctx)")
            }
            EagerExprVariant::EntityFp { route } => {
                self.gen_entity_route(route, EntityRouteRole::Caller)
            }
        }
    }
    fn gen_type_call_opn(
        &mut self,
        indent: Indent,
        ty: EntityRoutePtr,
        opds: &Vec<Arc<EagerExpr>>,
        ty_decl: &Arc<infer_decl::TyDecl>,
    ) {
        let type_call = ty_decl.opt_type_call.as_ref().unwrap();
        self.exec_within_context(
            RustCodeGenContext::ThisFieldWithPrefix { prefix: "__this_" },
            |this| {
                if type_call.keyword_parameters.len() > 0 {
                    this.gen_type_call_opn_with_keyword_parameters(indent, ty, opds, type_call)
                } else {
                    this.gen_type_call_opn_without_keyword_parameters(indent, ty, opds, type_call)
                }
            },
        )
    }

    fn gen_type_call_opn_without_keyword_parameters(
        &mut self,
        indent: Indent,
        ty: EntityRoutePtr,
        opds: &Vec<Arc<EagerExpr>>,
        type_call: &CallFormDecl,
    ) {
        let ty_defn = self.db.entity_defn(ty).unwrap();
        let ty_members = match ty_defn.variant {
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ref ty_members,
                ref variants,
                ty_kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                opt_static_visual_ty,
                ref opt_visual_stmts,
            } => ty_members,
            _ => panic!(),
        };
        self.gen_entity_route(ty, EntityRouteRole::Caller);
        self.write("::");
        self.write("__call__(");
        self.gen_arguments(indent, opds);
        msg_once!("variadics");
        match type_call.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { ty } => {
                if type_call.primary_parameters.len() + type_call.keyword_parameters.len() > 0 {
                    self.write(", ")
                }
                self.write("vec![]")
            }
        }
        self.write(")");
    }

    fn gen_type_call_opn_with_keyword_parameters(
        &mut self,
        indent: Indent,
        ty: EntityRoutePtr,
        opds: &Vec<Arc<EagerExpr>>,
        type_call: &CallFormDecl,
    ) {
        self.write("{\n");
        self.indent(indent + 8);
        let ty_defn = self.db.entity_defn(ty).unwrap();
        let ty_members = match ty_defn.variant {
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ref ty_members,
                ref variants,
                ty_kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                opt_static_visual_ty,
                ref opt_visual_stmts,
            } => ty_members,
            _ => panic!(),
        };
        for (i, (parameter, expr)) in
            std::iter::zip(type_call.primary_parameters.iter(), opds.iter()).enumerate()
        {
            self.write("let __this_");
            self.write(&parameter.ident);
            self.write(": ");
            self.gen_entity_route(parameter.ty, EntityRouteRole::Decl);
            self.write(" = ");
            self.gen_binding(expr);
            self.gen_expr(indent, expr);
            self.write(";");
            self.newline_indented(indent + 8);
        }
        for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
            self.write("let __this_");
            self.write(&parameter.ident);
            match ty_members.data()[type_call.primary_parameters.len() + i].variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    ..
                } => match field_variant {
                    FieldDefnVariant::StructDefault { default } => {
                        self.write(": ");
                        self.gen_entity_route(parameter.ty, EntityRouteRole::Decl);
                        self.write(" = ");
                        self.gen_expr(indent + 4, default);
                        self.write(";");
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
            self.newline_indented(indent + 8);
        }
        self.gen_entity_route(ty, EntityRouteRole::Caller);
        self.write("::");
        self.write("__call__(");
        for (i, parameter) in type_call.primary_parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write("__this_");
            self.write(&parameter.ident)
        }
        for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
            if i + type_call.primary_parameters.len() > 0 {
                self.write(", ")
            }
            self.write("__this_");
            self.write(&parameter.ident)
        }
        msg_once!("keyword arguments and more on variadics");
        match type_call.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { ty } => {
                if type_call.primary_parameters.len() + type_call.keyword_parameters.len() > 0 {
                    self.write(", ")
                }
                self.write("vec![]")
            }
        }
        self.write(")");
        self.newline_indented(indent + 4);
        self.write("}");
    }

    pub(super) fn gen_feature_return(&mut self, indent: Indent, result: &EagerExpr) {
        match result.qualified_ty.qual {
            EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
                self.write(
                    r#"__cache_feature(
        __ctx,
        __feature,
        Ok(("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#").__into_eval_value())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::EvalRef => {
                self.write(
                    r#"__cache_feature(
        __ctx,
        __feature,
        Ok(__EvalRef(&("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#")).into())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::PureRef
            | EagerExprQualifier::TempRef
            | EagerExprQualifier::TempRefMut => panic!(),
        }
    }

    pub(super) fn gen_lazy_field_return(&mut self, indent: Indent, result: &EagerExpr) {
        match result.qualified_ty.qual {
            EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
                self.write(
                    r#"__cache_lazy_field(
        __ctx,
        self,
        __uid,
        Ok(("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#").__into_eval_value())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::EvalRef => {
                self.write(
                    r#"__cache_lazy_field(
        __ctx,
        self,
        __uid,
        Ok(__EvalRef(&("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#")).into())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::PureRef
            | EagerExprQualifier::TempRef
            | EagerExprQualifier::TempRefMut => panic!(),
        }
    }

    fn gen_arguments(&mut self, indent: Indent, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_binding(expr);
            self.gen_expr(indent, expr)
        }
    }

    pub(super) fn gen_binding(&mut self, expr: &EagerExpr) {
        match expr.qualified_ty.qual.binding(expr.contract) {
            Binding::EvalRef => (),
            Binding::TempRef => self.write("&"),
            Binding::TempMut => self.write("&mut "),
            Binding::Move => (),
            Binding::Copy => (),
        }
    }

    fn gen_primitive_literal(&mut self, v: PrimitiveLiteralData) {
        match v {
            PrimitiveLiteralData::Integer(i) => {
                self.result.push_str(&i.to_string());
            }
            PrimitiveLiteralData::I32(i) => {
                self.result.push_str(&i.to_string());
                self.write("i32")
            }
            PrimitiveLiteralData::I64(i) => {
                self.result.push_str(&i.to_string());
                self.write("i64")
            }
            PrimitiveLiteralData::Float(f) => self.result.push_str(&f.to_string()),
            PrimitiveLiteralData::F32(f) => {
                self.result.push_str(&f.to_string());
                self.write("f32")
            }
            PrimitiveLiteralData::F64(f) => {
                self.result.push_str(&f.to_string());
                self.write("f64")
            }
            PrimitiveLiteralData::Bits(b) => self.result.push_str(&b.to_string()),
            PrimitiveLiteralData::B32(b) => {
                self.result.push_str(&b.to_string());
                self.write("u32")
            }
            PrimitiveLiteralData::B64(b) => {
                self.result.push_str(&b.to_string());
                self.write("u64")
            }
            PrimitiveLiteralData::Bool(b) => self.result.push_str(&b.to_string()),
            PrimitiveLiteralData::Void => self.result.push_str("()"),
        }
    }

    fn gen_suffix_opr(&mut self, opr: &SuffixOpr) {
        match opr {
            SuffixOpr::Incr => self.write(" += 1"),
            SuffixOpr::Decr => self.write(" -= 1"),
            SuffixOpr::AsTy(ty) => {
                self.write(" as ");
                self.gen_entity_route(ty.route, EntityRouteRole::Other)
            }
            SuffixOpr::BePattern(_) => todo!(),
        }
    }
}
