use husky_entity_semantics::{
    CallFormSource, DefinitionRepr, EnumVariantDefnVariant, FieldDefnVariant, TraitImplDefn,
};
use infer_decl::FieldDecl;
use word::CustomIdentifier;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_enum_defn(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        variants: &[Arc<EntityDefn>],
    ) {
        self.write("#[derive(Debug, Clone, Copy, PartialEq, Eq, __Serialize)]\n");
        self.write("pub(crate) enum ");
        self.write(&tyname);
        self.write(" {\n");
        for enum_variant_defn in variants {
            match enum_variant_defn.variant {
                EntityDefnVariant::EnumVariant { ident, ref variant } => match variant {
                    EnumVariantDefnVariant::Constant => {
                        self.result += "    ";
                        self.result += &enum_variant_defn.ident;
                        self.result += ",\n";
                    }
                },
                _ => panic!(""),
            }
        }
        self.result += "}\n";
        let ty_contains_eval_ref = self.db.entity_route_kind_contains_eval_ref(base_route.kind);
        self.gen_has_static_type_info_impl(base_route, tyname, ty_contains_eval_ref);
        self.gen_any_value_impl(base_route, tyname, ty_contains_eval_ref);
    }

    pub(super) fn gen_struct_defn(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        ty_members: &[Arc<EntityDefn>],
        trait_impls: &[Arc<TraitImplDefn>],
    ) {
        self.write("#[derive(Debug, Clone, PartialEq, __Serialize)]\n");
        self.result += "pub(crate) struct ";
        self.result += tyname.0;
        let ty_contains_eval_ref = self.db.entity_route_kind_contains_eval_ref(base_route.kind);
        if ty_contains_eval_ref {
            self.write("<'eval>")
        }
        self.result += " {\n";
        for member in ty_members {
            match member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason: contract,
                    ..
                } => {
                    match field_variant {
                        FieldDefnVariant::StructOriginal
                        | FieldDefnVariant::StructDefault { .. }
                        | FieldDefnVariant::StructDerivedEager { .. } => (),
                        FieldDefnVariant::StructDerivedLazy { .. } => break,
                        _ => panic!(),
                    }
                    self.result += "    pub(crate) ";
                    self.result += &member.ident;
                    self.result += ": ";
                    self.gen_entity_route(ty, EntityRouteRole::Decl);
                    self.write(",\n");
                }
                _ => break,
            }
        }
        self.result += "}\n";
        // impl member routines
        self.gen_struct_impls(
            base_route,
            tyname,
            ty_members,
            ty_contains_eval_ref,
            trait_impls,
        );
    }

    fn gen_struct_impls<'b>(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        ty_members: &[Arc<EntityDefn>],
        ty_contains_eval_ref: bool,
        trait_impls: &[Arc<TraitImplDefn>],
    ) {
        if ty_contains_eval_ref {
            self.write("\nimpl<'eval> ");
        } else {
            self.write("\nimpl ");
        }
        self.write(&tyname);
        if ty_contains_eval_ref {
            self.write("<'eval>")
        }
        self.write(" {\n");
        self.gen_struct_type_call(ty_members);
        let mut start_flag = true;
        for ty_member in ty_members {
            self.gen_ty_member_impl(ty_contains_eval_ref, ty_member, &mut start_flag)
        }

        self.write("}\n");

        self.gen_has_static_type_info_impl(base_route, tyname, ty_contains_eval_ref);

        self.gen_any_value_impl(base_route, tyname, ty_contains_eval_ref);

        for trait_impl in trait_impls {
            self.gen_trait_impl(tyname, trait_impl)
        }
    }

    fn gen_struct_type_call(&mut self, ty_members: &[Arc<EntityDefn>]) {
        self.write("    pub(crate) fn __call__(");
        for (i, ty_member) in ty_members.iter().enumerate() {
            match ty_member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => match field_variant {
                    FieldDefnVariant::StructOriginal | FieldDefnVariant::StructDefault { .. } => {
                        if i > 0 {
                            self.write(", ")
                        }
                        self.write(&ty_member.ident);
                        self.write(": ");
                        self.gen_entity_route(ty, EntityRouteRole::Decl)
                    }
                    FieldDefnVariant::StructDerivedEager { .. }
                    | FieldDefnVariant::StructDerivedLazy { .. } => break,
                    FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => {
                        panic!()
                    }
                },
                _ => break,
            }
        }
        self.write(") -> Self {\n");
        for (i, ty_member) in ty_members.iter().enumerate() {
            match ty_member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => match field_variant {
                    FieldDefnVariant::StructOriginal | FieldDefnVariant::StructDefault { .. } => (),
                    FieldDefnVariant::StructDerivedEager { ref derivation, .. } => {
                        self.indent(8);
                        self.write("let ");
                        self.write(&ty_member.ident);
                        self.write(" = ");
                        self.exec_within_context(
                            RustCodeGenContext::ThisFieldWithPrefix { prefix: "" },
                            |this| this.gen_expr(8, derivation),
                        );
                        self.write(";");
                        self.newline();
                    }
                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                    FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => {
                        panic!()
                    }
                },
                _ => break,
            }
        }
        self.write("        Self { ");
        for (i, ty_member) in ty_members.iter().enumerate() {
            match ty_member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => match field_variant {
                    FieldDefnVariant::StructOriginal
                    | FieldDefnVariant::StructDefault { .. }
                    | FieldDefnVariant::StructDerivedEager { .. } => {
                        if i > 0 {
                            self.write(", ")
                        }
                        self.write(&ty_member.ident)
                    }
                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                    FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => {
                        panic!()
                    }
                },
                _ => break,
            }
        }
        self.write(" }\n");
        self.write("    }\n");
    }

    fn gen_ty_member_impl(
        &mut self,
        ty_contains_eval_ref: bool,
        ty_member: &EntityDefn,
        start_flag: &mut bool,
    ) {
        match ty_member.variant {
            EntityDefnVariant::Method {
                spatial_parameters: ref generic_parameters,
                this_liason: this_contract,
                ref parameters,
                output_ty,
                output_liason,
                opt_source: Some(ref source),
                ..
            } => {
                match source {
                    CallFormSource::Func { .. } | CallFormSource::Proc { .. } => (),
                    CallFormSource::Lazy { .. } => return,
                    CallFormSource::Static(_) => panic!(),
                }
                if *start_flag {
                    *start_flag = false
                } else {
                    self.write("\n")
                }
                self.write("    pub(crate) fn ");
                self.write(&ty_member.ident);
                self.write("(");
                match this_contract {
                    ParameterLiason::Pure => self.write("&self"),
                    ParameterLiason::EvalRef => todo!(),
                    ParameterLiason::Move => todo!(),
                    ParameterLiason::TempRefMut => todo!(),
                    ParameterLiason::MoveMut => todo!(),
                    ParameterLiason::MemberAccess => todo!(),
                    ParameterLiason::TempRef => todo!(),
                }
                for parameter in parameters.iter() {
                    self.write(", ");
                    self.gen_parameter(parameter)
                }
                self.write(") -> ");
                self.gen_entity_route(output_ty.route, EntityRouteRole::Decl);
                self.write(" {\n");
                self.gen_call_form_source(source);
                self.write("    }\n");
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => self.gen_func_defn(4, ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Proc {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => self.gen_proc_defn(4, ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Function { .. } => todo!(),
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => match field_variant {
                FieldDefnVariant::StructOriginal
                | FieldDefnVariant::StructDefault { .. }
                | FieldDefnVariant::StructDerivedEager { .. } => (),
                FieldDefnVariant::StructDerivedLazy { defn_repr } => match **defn_repr {
                    DefinitionRepr::LazyExpr { ref expr } => todo!(),
                    DefinitionRepr::LazyBlock { ref stmts, ty } => (),
                    DefinitionRepr::FuncBlock {
                        route,
                        file,
                        range,
                        ref stmts,
                        ty,
                    } => {
                        self.write("pub(crate) fn ");
                        let ident = ty_member.ident;
                        self.write(&ident);
                        if !ty_contains_eval_ref {
                            self.write("<'eval>")
                        }
                        self.write("(&'eval self, __ctx: &__EvalContext<'eval>) -> &'eval ");
                        self.gen_entity_route(ty.route.deref_route(), EntityRouteRole::Decl);
                        let route = ty_member.base_route;
                        self.write(&format!(
                            r#" {{
    let __uid = entity_uid!(__ctx, "{route:?}");
    if let Some(__result) = __opt_cached_lazy_field(__ctx, self, __uid) {{
        return __result.unwrap();
    }}
"#,
                        ));
                        self.gen_func_stmts(stmts);
                        self.write("    }\n");
                    }
                    DefinitionRepr::ProcBlock {
                        file,
                        range,
                        ref stmts,
                        ty,
                    } => todo!(),
                },
                FieldDefnVariant::RecordOriginal => (),
                FieldDefnVariant::RecordDerived { defn_repr } => (),
            },
            _ => (),
        }
    }

    fn gen_has_static_type_info_impl(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        ty_contains_eval_ref: bool,
    ) {
        if ty_contains_eval_ref {
            self.write("\nimpl<'eval> __HasStaticTypeInfo for ");
        } else {
            self.write("\nimpl __HasStaticTypeInfo for ");
        }
        self.write(&tyname);
        if ty_contains_eval_ref {
            self.write("<'eval>")
        }
        let static_self = if ty_contains_eval_ref {
            format!("{tyname}<'static>")
        } else {
            tyname.as_str().to_owned()
        };
        self.write(&format!(
            r#" {{
    type __StaticSelf = {static_self};

    fn __static_type_name() -> std::borrow::Cow<'static, str> {{
        "{base_route:?}".into()
    }}
}}
"#
        ));
    }

    fn gen_any_value_impl(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        ty_contains_eval_ref: bool,
    ) {
        self.write("\nimpl<'eval> __AnyValue<'eval> for ");
        self.write(&tyname);
        if ty_contains_eval_ref {
            self.write("<'eval>")
        }
        let into_eval_value_impl = if self.db.is_copyable(base_route).unwrap() {
            "todo!()"
        } else {
            "__EvalValue::Owned(__OwnedValue::new(self))"
        };
        self.write(&format!(
            r#" {{
    fn __print_short(&self) -> String {{
        "{{ ... }}".to_owned()
    }}

    fn __to_json_value(&self) -> __JsonValue {{
        serde_json::value::to_value(self).unwrap()
    }}

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {{
        self
    }}

    fn __static_ty() -> __EntityRoutePtr {{
        __ty_route_from_static_binded::<Self>("{base_route:?}")
    }}

    fn __into_eval_value(self) -> __EvalValue<'eval> {{
        {into_eval_value_impl}
    }}

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {{
        todo!()
    }}
}}
"#,
        ));
    }

    fn gen_trait_impl(&mut self, tyname: CustomIdentifier, trait_impl: &TraitImplDefn) {
        todo!()
    }
}
