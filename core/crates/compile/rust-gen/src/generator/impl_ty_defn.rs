use infer_decl::FieldDecl;
use semantics_entity::{CallFormSource, EnumVariantDefnVariant, FieldDefnVariant, TraitImplDefn};
use word::CustomIdentifier;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_enum_defn(&mut self, tyname: CustomIdentifier, variants: &[Arc<EntityDefn>]) {
        self.write("enum ");
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
        tyname: CustomIdentifier,
        ty_members: &[Arc<EntityDefn>],
        trait_impls: &[Arc<TraitImplDefn>],
    ) {
        self.result += "pub struct ";
        self.result += tyname.0;
        self.result += " {\n";
        let mut member_iter = ty_members.iter().peekable();
        while let Some(member) = member_iter.peek() {
            match member.variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason: contract,
                    ..
                } => {
                    match field_variant {
                        FieldDefnVariant::StructOriginal => (),
                        FieldDefnVariant::StructDerivedLazy { .. } => break,
                        _ => panic!(),
                    }
                    self.result += "    pub(crate) ";
                    self.result += &member.ident;
                    self.result += ": ";
                    self.gen_entity_route(ty);
                    self.write(",\n");
                }
                EntityDefnVariant::Method { .. } => break,
                _ => panic!(),
            }
            member_iter.next();
        }
        self.result += "}\n";
        // impl member routines
        self.gen_struct_methods(tyname, member_iter);
        for trait_impl in trait_impls {
            self.gen_trait_impl(tyname, trait_impl)
        }
    }

    // fn gen_struct_call(&mut self, fields: &[FieldDefn]) {
    //     self.write("    pub(crate) fn __call__(");
    //     for (i, field) in fields.iter().enumerate() {
    //         if i > 0 {
    //             self.write(", ")
    //         }
    //         self.write(&field.ident);
    //         self.write(": ");
    //         match field.contract {
    //             FieldContract::Own => (),
    //             FieldContract::Ref => todo!(),
    //             FieldContract::LazyOwn => todo!(),
    //         }
    //         self.gen_scope(field.ty)
    //     }
    //     self.write(") -> Self {\n");
    //     self.write("        Self {");
    //     for (i, field) in fields.iter().enumerate() {
    //         if i > 0 {
    //             self.write(", ")
    //         }
    //         self.write(&field.ident)
    //     }
    //     self.write("}\n");
    //     self.write("    }\n");
    // }

    fn gen_struct_methods<'b>(
        &mut self,
        tyname: CustomIdentifier,
        methods: impl Iterator<Item = &'b Arc<EntityDefn>>,
    ) {
        self.write("\nimpl ");
        self.write(&tyname);
        self.write(" {\n");
        let mut start_flag = true;
        for method in methods {
            match method.variant {
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
                        CallFormSource::Lazy { .. } => continue,
                        CallFormSource::Static(_) => panic!(),
                    }
                    if start_flag {
                        start_flag = false
                    } else {
                        self.write("\n")
                    }
                    self.write("    pub(crate) fn ");
                    self.write(&method.ident);
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
                    for input_placeholder in parameters.iter() {
                        self.write(", ");
                        self.write(&input_placeholder.ranged_ident.ident);
                        self.write(": ");
                        match input_placeholder.ranged_liason.liason {
                            ParameterLiason::Pure => {
                                if !self
                                    .db
                                    .is_copyable(input_placeholder.ranged_ty.route)
                                    .unwrap()
                                {
                                    self.write("&")
                                }
                            }
                            ParameterLiason::EvalRef => todo!(),
                            ParameterLiason::Move => todo!(),
                            ParameterLiason::TempRefMut => todo!(),
                            ParameterLiason::MoveMut => todo!(),
                            ParameterLiason::MemberAccess => todo!(),
                            ParameterLiason::TempRef => todo!(),
                        }
                        self.gen_entity_route(input_placeholder.ranged_ty.route);
                    }
                    self.write(") -> ");
                    self.gen_entity_route(output_ty.route);
                    self.write(" {\n");
                    match source {
                        CallFormSource::Func { stmts } => self.gen_func_stmts(stmts, 8),
                        CallFormSource::Proc { stmts } => self.gen_proc_stmts(stmts, 8),
                        CallFormSource::Lazy { stmts } => todo!(),
                        CallFormSource::Static(_) => todo!(),
                    }
                    self.write("    }\n");
                }
                _ => (),
            }
        }
        self.write("}\n");
    }

    fn gen_trait_impl(&mut self, tyname: CustomIdentifier, trait_impl: &TraitImplDefn) {
        todo!()
    }
}
