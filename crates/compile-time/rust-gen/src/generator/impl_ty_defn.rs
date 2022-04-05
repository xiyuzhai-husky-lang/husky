use semantics_entity::{EnumVariantKind, MembRoutineDefn, MembRoutineKind};
use syntax_types::MembAccessSignature;
use vm::{InputContract, MembAccessContract};
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_enum_defn(
        &mut self,
        tyname: CustomIdentifier,
        variants: &[(CustomIdentifier, EnumVariantKind)],
    ) {
        self.write("enum ");
        self.write(&tyname);
        self.write(" {\n");
        for (variant_ident, variant_kind) in variants {
            match variant_kind {
                EnumVariantKind::Constant => {
                    self.result += "    ";
                    self.result += variant_ident.0;
                    self.result += ",\n";
                }
            }
        }
        self.result += "}\n";
    }

    pub(super) fn gen_struct_defn(
        &mut self,
        tyname: CustomIdentifier,
        memb_vars: &[(CustomIdentifier, MembAccessSignature)],
        memb_routines: &[(CustomIdentifier, MembRoutineDefn)],
    ) {
        self.result += "pub struct ";
        self.result += tyname.0;
        self.result += " {\n";
        for (memb_var_ident, memb_var_signature) in memb_vars {
            self.result += "    pub(crate) ";
            self.result += memb_var_ident.0;
            self.result += ": ";
            match memb_var_signature.contract {
                MembAccessContract::Own => (),
                MembAccessContract::Ref => todo!(),
                MembAccessContract::LazyOwn => todo!(),
            }
            self.gen_scope(memb_var_signature.ty);
            self.result += ",\n";
        }
        self.result += "}\n";
        // impl member routines
        self.write("\nimpl ");
        self.write(&tyname);
        self.write(" {\n");
        self.gen_struct_call(memb_vars);
        self.gen_struct_memb_routines(memb_routines);
        self.write("}\n");
    }

    fn gen_struct_call(&mut self, memb_vars: &[(CustomIdentifier, MembAccessSignature)]) {
        self.write("    pub(crate) fn __call__(");
        for (i, (memb_var_ident, memb_var_signature)) in memb_vars.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&memb_var_ident);
            self.write(": ");
            match memb_var_signature.contract {
                MembAccessContract::Own => (),
                MembAccessContract::Ref => todo!(),
                MembAccessContract::LazyOwn => todo!(),
            }
            self.gen_scope(memb_var_signature.ty)
        }
        self.write(") -> Self {\n");
        self.write("        Self {");
        for (i, (memb_var_ident, _)) in memb_vars.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&memb_var_ident)
        }
        self.write("}\n");
        self.write("    }\n");
    }

    fn gen_struct_memb_routines(&mut self, memb_routines: &[(CustomIdentifier, MembRoutineDefn)]) {
        for (memb_routine_ident, memb_routine) in memb_routines {
            self.write("\n    pub(crate) fn ");
            self.write(&memb_routine_ident);
            self.write("(");
            match memb_routine.this_contract {
                InputContract::Pure => self.write("&self"),
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
            }
            for input_placeholder in memb_routine.input_placeholders.iter() {
                self.write(", ");
                self.write(&input_placeholder.ident);
                self.write(": ");
                match input_placeholder.contract {
                    InputContract::Pure => {
                        if !self.db.is_copyable(input_placeholder.ranged_ty.scope) {
                            self.write("&")
                        }
                    }
                    InputContract::GlobalRef => todo!(),
                    InputContract::Move => todo!(),
                    InputContract::BorrowMut => todo!(),
                    InputContract::MoveMut => todo!(),
                    InputContract::Exec => todo!(),
                }
                self.gen_scope(input_placeholder.ranged_ty.scope);
            }
            self.write(") -> ");
            self.gen_scope(memb_routine.output.scope);
            self.write(" {\n");
            match memb_routine.kind {
                MembRoutineKind::Func { ref stmts } => self.gen_decl_stmts(stmts, 8),
                MembRoutineKind::Proc { ref stmts } => todo!(),
            }
            self.write("    }\n");
        }
    }
}
