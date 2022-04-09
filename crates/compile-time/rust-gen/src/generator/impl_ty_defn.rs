use infer_decl::FieldDecl;
use semantics_entity::{
    EnumVariantDefn, EnumVariantDefnVariant, FieldDefn, MethodDefn, MethodKind,
};
use vm::{InputContract, MembAccessContract};
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_enum_defn(&mut self, tyname: CustomIdentifier, variants: &[EnumVariantDefn]) {
        self.write("enum ");
        self.write(&tyname);
        self.write(" {\n");
        for enum_variant_defn in variants {
            match enum_variant_defn.variant {
                EnumVariantDefnVariant::Constant => {
                    self.result += "    ";
                    self.result += &enum_variant_defn.ident;
                    self.result += ",\n";
                }
            }
        }
        self.result += "}\n";
    }

    pub(super) fn gen_struct_defn(
        &mut self,
        tyname: CustomIdentifier,
        fields: &[FieldDefn],
        methods: &[MethodDefn],
    ) {
        self.result += "pub struct ";
        self.result += tyname.0;
        self.result += " {\n";
        for field in fields {
            self.result += "    pub(crate) ";
            self.result += &field.ident;
            self.result += ": ";
            todo!();
            // match field.contract {
            //     MembAccessContract::Own => (),
            //     MembAccessContract::Ref => todo!(),
            //     MembAccessContract::LazyOwn => todo!(),
            // }
            self.gen_scope(field.ty);
            self.result += ",\n";
        }
        self.result += "}\n";
        // impl member routines
        self.write("\nimpl ");
        self.write(&tyname);
        self.write(" {\n");
        self.gen_struct_call(fields);
        self.gen_struct_field_routines(methods);
        self.write("}\n");
    }

    fn gen_struct_call(&mut self, fields: &[FieldDefn]) {
        self.write("    pub(crate) fn __call__(");
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&field.ident);
            self.write(": ");
            match field.contract() {
                MembAccessContract::Own => (),
                MembAccessContract::Ref => todo!(),
                MembAccessContract::LazyOwn => todo!(),
            }
            self.gen_scope(field.ty)
        }
        self.write(") -> Self {\n");
        self.write("        Self {");
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&field.ident)
        }
        self.write("}\n");
        self.write("    }\n");
    }

    fn gen_struct_field_routines(&mut self, methods: &[MethodDefn]) {
        for method in methods {
            self.write("\n    pub(crate) fn ");
            self.write(&method.ident);
            self.write("(");
            match method.this_contract {
                InputContract::Pure => self.write("&self"),
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
            }
            for input_placeholder in method.input_placeholders.iter() {
                self.write(", ");
                self.write(&input_placeholder.ident);
                self.write(": ");
                match input_placeholder.contract {
                    InputContract::Pure => {
                        if !self.db.is_copyable(input_placeholder.ranged_ty.route) {
                            self.write("&")
                        }
                    }
                    InputContract::GlobalRef => todo!(),
                    InputContract::Move => todo!(),
                    InputContract::BorrowMut => todo!(),
                    InputContract::MoveMut => todo!(),
                    InputContract::Exec => todo!(),
                }
                self.gen_scope(input_placeholder.ranged_ty.route);
            }
            self.write(") -> ");
            self.gen_scope(method.output.route);
            self.write(" {\n");
            match method.kind {
                MethodKind::Func { ref stmts } => self.gen_func_stmts(stmts, 8),
                MethodKind::Proc { ref stmts } => todo!(),
            }
            self.write("    }\n");
        }
    }
}
