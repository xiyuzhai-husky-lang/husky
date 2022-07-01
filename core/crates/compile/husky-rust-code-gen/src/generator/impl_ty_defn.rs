use infer_decl::FieldDecl;
use semantics_entity::{CallFormSource, EnumVariantDefnVariant, FieldDefnVariant, TraitImplDefn};
use word::CustomIdentifier;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_enum_defn(&mut self, tyname: CustomIdentifier, variants: &[Arc<EntityDefn>]) {
        self.write("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        self.write("pub enum ");
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
    }

    pub(super) fn gen_struct_defn(
        &mut self,
        base_route: EntityRoutePtr,
        tyname: CustomIdentifier,
        ty_members: &[Arc<EntityDefn>],
        trait_impls: &[Arc<TraitImplDefn>],
    ) {
        self.result += "pub struct ";
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
        self.gen_struct_impls(tyname, ty_members, ty_contains_eval_ref, trait_impls);
    }

    fn gen_struct_impls<'b>(
        &mut self,
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
        self.gen_struct_call(ty_members);
        let mut start_flag = true;
        for ty_member in ty_members {
            self.gen_ty_member_impl(ty_member, &mut start_flag)
        }

        for trait_impl in trait_impls {
            self.gen_trait_impl(tyname, trait_impl)
        }

        self.write("}\n");
    }

    fn gen_struct_call(&mut self, ty_members: &[Arc<EntityDefn>]) {
        self.write("    pub(crate) fn __call__(");
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
                        self.write(&ty_member.ident);
                        self.write(": ");
                        self.gen_entity_route(ty, EntityRouteRole::Decl)
                    }
                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                    FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => {
                        panic!()
                    }
                },
                _ => break,
            }
        }
        self.write(") -> Self {\n");
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

    fn gen_ty_member_impl(&mut self, ty_member: &EntityDefn, start_flag: &mut bool) {
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
            } => self.gen_func_defn(ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Proc {
                ref generic_parameters,
                ref parameters,
                output,
                ref stmts,
            } => self.gen_proc_defn(ty_member.base_route, parameters, output.route, stmts),
            EntityDefnVariant::Function { .. } => todo!(),
            _ => (),
        }
    }

    fn gen_trait_impl(&mut self, tyname: CustomIdentifier, trait_impl: &TraitImplDefn) {
        todo!()
    }
}
