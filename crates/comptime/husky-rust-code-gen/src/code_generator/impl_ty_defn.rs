use husky_entity_semantics::{
    CallFormSource, DefinitionRepr, EnumVariantDefnVariant, FieldDefnVariant, TraitImplDefn,
};
use husky_word::{Identifier, RootBuiltinIdentifier};

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_enum_defn(
        &mut self,
        base_route: Term,
        tyname: Identifier,
        variants: &[Arc<EntityDefn>],
    ) {
        self.write(
            r#"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
"#,
        );
        self.write(
            r#"
pub(crate) enum "#,
        );
        self.write(&tyname);
        self.write(" {\n");
        for enum_variant_defn in variants {
            match enum_variant_defn.variant {
                EntityDefnVariant::EnumVariant {
                    enum_variant_defn_variant: ref variant,
                } => match variant {
                    EnumVariantDefnVariant::Constant => {
                        self.result += "    ";
                        self.result += &enum_variant_defn.ident;
                        self.result += ",\n";
                    }
                },
                _ => panic!(""),
            }
        }
        self.result += "}";
        self.write(&format!(
            r#"
        
impl From<i32> for {tyname} {{
    fn from(__raw: i32) -> Self {{
        match __raw {{"#,
        ));
        for (i, variant) in variants.iter().enumerate() {
            self.write(&format!(
                r#"
            {i} => "#
            ));
            self.gen_entity_route(variant.base_route, EntityRouteRole::Decl);
            self.write(",")
        }
        self.write(
            r#"
            _ => panic!(),
        }
    }
}"#,
        );
        let ty_contains_eval_ref = self.db.entity_route_variant_contains_eval_ref(base_route);
        self.gen_has_static_type_info_impl(base_route, tyname, ty_contains_eval_ref);
    }

    pub(super) fn gen_struct_defn(
        &mut self,
        base_route: Term,
        tyname: Identifier,
        ty_members: &[Arc<EntityDefn>],
        trait_impls: &[Arc<TraitImplDefn>],
    ) {
        self.write("#[derive(Debug, Clone, PartialEq)]\n");
        self.result += "pub(crate) struct ";
        self.result += tyname.as_str();
        let ty_contains_eval_ref = self.db.entity_route_variant_contains_eval_ref(base_route);
        if ty_contains_eval_ref {
            self.write("<'eval>")
        }
        self.result += " {\n";
        for member in ty_members {
            match member.variant {
                EntityDefnVariant::TyField {
                    field_ty: ty,
                    ref field_variant,
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
        base_route: Term,
        tyname: Identifier,
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

        for trait_impl in trait_impls {
            self.gen_trait_impl(trait_impl)
        }
    }

    fn gen_struct_type_call(&mut self, ty_members: &[Arc<EntityDefn>]) {
        self.write("    pub(crate) fn __call__(");
        for (i, ty_member) in ty_members.iter().enumerate() {
            match ty_member.variant {
                EntityDefnVariant::TyField {
                    field_ty,
                    ref field_variant,
                    ..
                } => match field_variant {
                    FieldDefnVariant::StructOriginal | FieldDefnVariant::StructDefault { .. } => {
                        if i > 0 {
                            self.write(", ")
                        }
                        self.write(&ty_member.ident);
                        self.write(": ");
                        self.gen_entity_route(field_ty, EntityRouteRole::Decl)
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
        for ty_member in ty_members.iter() {
            match ty_member.variant {
                EntityDefnVariant::TyField {
                    ref field_variant, ..
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
                    ref field_variant, ..
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
                this_modifier: this_contract,
                ref parameters,
                return_ty,
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
                    ParameterModifier::None => self.write("&self"),
                    ParameterModifier::EvalRef => todo!(),
                    ParameterModifier::Owned => todo!(),
                    ParameterModifier::TempRefMut => todo!(),
                    ParameterModifier::OwnedMut => todo!(),
                    ParameterModifier::MemberAccess => todo!(),
                    ParameterModifier::TempRef => todo!(),
                }
                for parameter in parameters.iter() {
                    self.write(", ");
                    self.gen_parameter(parameter)
                }
                self.write(") -> ");
                self.gen_entity_route(return_ty.route, EntityRouteRole::Decl);
                self.write(" {\n");
                self.gen_call_form_source(source);
                self.write("    }\n");
            }
            EntityDefnVariant::Func {
                ref parameters,
                output,
                ref stmts,
                ..
            } => self.gen_func_defn(4, ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Proc {
                ref parameters,
                output,
                ref stmts,
                ..
            } => self.gen_proc_defn(4, ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Function { .. } => todo!(),
            EntityDefnVariant::TyField {
                ref field_variant, ..
            } => match field_variant {
                FieldDefnVariant::StructOriginal
                | FieldDefnVariant::StructDefault { .. }
                | FieldDefnVariant::StructDerivedEager { .. } => (),
                FieldDefnVariant::StructDerivedLazy { defn_repr } => match **defn_repr {
                    DefinitionRepr::LazyExpr { .. } => todo!(),
                    DefinitionRepr::LazyBlock { .. } => (),
                    DefinitionRepr::FuncBlock {
                        ref stmts,
                        return_ty,
                        ..
                    } => {
                        self.write("pub(crate) fn ");
                        let ident = ty_member.ident;
                        self.write(&ident);
                        if !ty_contains_eval_ref {
                            self.write("<'eval>")
                        }
                        if return_ty.route.is_option() {
                            todo!()
                        }
                        self.write("(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval ");
                        self.gen_entity_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                        let route = ty_member.base_route;
                        let mangled_return_ty_vtable =
                            self.db.mangled_intrinsic_ty_vtable(return_ty.route);
                        self.write(&format!(
                            r#" {{
    let __uid = entity_uid!(__ctx, "{route:?}");
    if let Some(__result) = __ctx.opt_cached_lazy_field(
        self as *const _ as *const std::ffi::c_void,
        __uid
    ) {{
        return __result
            .unwrap()
            .downcast_{}eval_ref(&__registration__::{mangled_return_ty_vtable});
    }}
"#,
                            match return_ty.route.is_option() {
                                true => "opt_",
                                false => "",
                            }
                        ));
                        self.gen_func_stmts(stmts);
                        self.write("    }\n");
                    }
                    DefinitionRepr::ProcBlock {
                        ref stmts,
                        return_ty,
                        ..
                    } => {
                        self.write("pub(crate) fn ");
                        let ident = ty_member.ident;
                        self.write(&ident);
                        if !ty_contains_eval_ref {
                            self.write("<'eval>")
                        }
                        if return_ty.route.is_option() {
                            todo!()
                        }
                        self.write("(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval ");
                        self.gen_entity_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                        let route = ty_member.base_route;
                        let mangled_return_ty_vtable =
                            self.db.mangled_intrinsic_ty_vtable(return_ty.route);
                        self.write(&format!(
                            r#" {{
    let __uid = entity_uid!(__ctx, "{route:?}");
    if let Some(__result) = __ctx.opt_cached_lazy_field(
        self as *const _ as *const std::ffi::c_void,
        __uid
    ) {{
        return __result
            .unwrap()
            .downcast_{}eval_ref(&__registration__::{mangled_return_ty_vtable});
    }}
"#,
                            match return_ty.route.is_option() {
                                true => "opt_",
                                false => "",
                            }
                        ));
                        self.gen_proc_stmts(stmts);
                        self.write("    }\n");
                    }
                },
                FieldDefnVariant::RecordOriginal => (),
                FieldDefnVariant::RecordDerived { .. } => (),
            },
            _ => (),
        }
    }

    fn gen_has_static_type_info_impl(
        &mut self,
        base_route: Term,
        tyname: Identifier,
        ty_contains_eval_ref: bool,
    ) {
        if ty_contains_eval_ref {
            self.write("\nimpl<'eval> __StaticInfo for ");
        } else {
            self.write("\nimpl __StaticInfo for ");
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

    fn __static_typename() -> std::borrow::Cow<'static, str> {{
        "{base_route:?}".into()
    }}
    
    unsafe fn __transmute_static(self) -> Self::__StaticSelf {{
        std::mem::transmute(self)
    }}
}}
"#
        ));
    }

    fn gen_trait_impl(&mut self, trait_impl: &TraitImplDefn) {
        if trait_impl.trai == RootBuiltinIdentifier::CopyTrait.into() {
            return;
        }
        if trait_impl.trai == RootBuiltinIdentifier::CloneTrait.into() {
            return;
        }
        todo!()
    }
}
