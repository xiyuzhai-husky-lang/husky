use infer_decl::FieldDecl;
use semantics_entity::{EnumVariantDefnVariant, FieldDefnVariant};
use vm::FieldContract;
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

    pub(super) fn gen_struct_defn(&mut self, tyname: CustomIdentifier, members: &[EntityDefn]) {
        self.result += "pub struct ";
        self.result += tyname.0;
        self.result += " {\n";
        for member in members {
            self.result += "    pub(crate) ";
            self.result += &member.ident;
            self.result += ": ";
            match member.variant {
                EntityDefnVariant::TypeField {
                    ty,
                    ref field_variant,
                    contract,
                } => {
                    match contract {
                        FieldContract::Own => (),
                        FieldContract::GlobalRef => todo!(),
                        FieldContract::LazyOwn => todo!(),
                    }

                    self.gen_entity_route(ty);
                    match field_variant {
                        FieldDefnVariant::StructOriginal => (),
                        FieldDefnVariant::RecordOriginal => (),
                        FieldDefnVariant::StructDerived { ref stmts } => todo!(),
                        FieldDefnVariant::RecordDerived { ref stmts } => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
        self.result += "}\n";
        // impl member routines
        self.write("\nimpl ");
        self.write(&tyname);
        self.write(" {\n");
        self.write("}\n");
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

    // fn gen_struct_field_routines(&mut self, methods: &[MethodDefn]) {
    //     for method in methods {
    //         self.write("\n    pub(crate) fn ");
    //         self.write(&method.ident);
    //         self.write("(");
    //         match method.this_contract {
    //             InputContract::Pure => self.write("&self"),
    //             InputContract::GlobalRef => todo!(),
    //             InputContract::Move => todo!(),
    //             InputContract::BorrowMut => todo!(),
    //             InputContract::MoveMut => todo!(),
    //             InputContract::Exec => todo!(),
    //         }
    //         for input_placeholder in method.input_placeholders.iter() {
    //             self.write(", ");
    //             self.write(&input_placeholder.ident);
    //             self.write(": ");
    //             match input_placeholder.contract {
    //                 InputContract::Pure => {
    //                     if !self.db.is_copyable(input_placeholder.ranged_ty.route) {
    //                         self.write("&")
    //                     }
    //                 }
    //                 InputContract::GlobalRef => todo!(),
    //                 InputContract::Move => todo!(),
    //                 InputContract::BorrowMut => todo!(),
    //                 InputContract::MoveMut => todo!(),
    //                 InputContract::Exec => todo!(),
    //             }
    //             self.gen_scope(input_placeholder.ranged_ty.route);
    //         }
    //         self.write(") -> ");
    //         self.gen_scope(method.output.route);
    //         self.write(" {\n");
    //         match method.variant {
    //             MethodDefnVariant::Func { ref stmts } => self.gen_func_stmts(stmts, 8),
    //             MethodDefnVariant::Proc { ref stmts } => todo!(),
    //             MethodDefnVariant::Pattern { ref stmts } => todo!(),
    //         }
    //         self.write("    }\n");
    //     }
    // }
}
