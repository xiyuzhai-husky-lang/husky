use infer_decl::FieldDecl;
use semantics_entity::{
    EnumVariantDefnVariant, FieldDefnVariant, MethodDefnVariant, MethodSource, TraitImplDefn,
};
use vm::{FieldLiason, InputLiason};
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
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
                EntityDefnVariant::TypeField {
                    ty,
                    ref fieldiant,
                    contract,
                } => {
                    match fieldiant {
                        FieldDefnVariant::StructOriginal => (),
                        FieldDefnVariant::StructDerived { .. } => break,
                        _ => panic!(),
                    }
                    self.result += "    pub(crate) ";
                    self.result += &member.ident;
                    self.result += ": ";
                    match contract {
                        FieldLiason::Own => (),
                        FieldLiason::GlobalRef => todo!(),
                        FieldLiason::LazyOwn => todo!(),
                    }
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
                    ref generic_parameters,
                    this_contract,
                    parameters: ref parameters,
                    output_ty,
                    output_liason,
                    ref method_variant,
                } => {
                    match method_variant {
                        MethodDefnVariant::TypeMethod { ty, method_source } => {
                            match method_source {
                                MethodSource::Func { .. } | MethodSource::Proc { .. } => (),
                                MethodSource::Pattern { .. } => continue,
                                MethodSource::Static(_) => panic!(),
                            }
                        }
                        _ => panic!(),
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
                        InputLiason::Pure => self.write("&self"),
                        InputLiason::GlobalRef => todo!(),
                        InputLiason::Move => todo!(),
                        InputLiason::BorrowMut => todo!(),
                        InputLiason::MoveMut => todo!(),
                        InputLiason::MemberAccess => todo!(),
                    }
                    for input_placeholder in parameters.iter() {
                        self.write(", ");
                        self.write(&input_placeholder.ranged_ident.ident);
                        self.write(": ");
                        match input_placeholder.contract {
                            InputLiason::Pure => {
                                if !self
                                    .db
                                    .is_copyable(input_placeholder.ranged_ty.route)
                                    .unwrap()
                                {
                                    self.write("&")
                                }
                            }
                            InputLiason::GlobalRef => todo!(),
                            InputLiason::Move => todo!(),
                            InputLiason::BorrowMut => todo!(),
                            InputLiason::MoveMut => todo!(),
                            InputLiason::MemberAccess => todo!(),
                        }
                        self.gen_entity_route(input_placeholder.ranged_ty.route);
                    }
                    self.write(") -> ");
                    self.gen_entity_route(output_ty.route);
                    self.write(" {\n");
                    match method_variant {
                        MethodDefnVariant::TypeMethod { ty, method_source } => {
                            match method_source {
                                MethodSource::Func { stmts } => self.gen_func_stmts(stmts, 8),
                                MethodSource::Proc { stmts } => self.gen_proc_stmts(stmts, 8),
                                MethodSource::Pattern { stmts } => todo!(),
                                MethodSource::Static(_) => todo!(),
                            }
                        }
                        _ => panic!(),
                    }
                    self.write("    }\n");
                }
                _ => panic!(),
            }
        }
        self.write("}\n");
    }

    fn gen_trait_impl(&mut self, tyname: CustomIdentifier, trait_impl: &TraitImplDefn) {
        todo!()
    }
}
