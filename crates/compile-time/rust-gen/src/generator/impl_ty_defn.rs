use semantics_entity::{EnumVariantKind, MembRoutine, MembRoutineKind};
use syntax_types::MembVarSignature;
use vm::{InputContract, MembVarContract};
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
        memb_vars: &[(CustomIdentifier, MembVarSignature)],
        memb_routines: &[(CustomIdentifier, MembRoutine)],
    ) {
        self.result += "pub struct ";
        self.result += tyname.0;
        self.result += " {\n";
        for (memb_var_ident, memb_var_signature) in memb_vars {
            self.result += "    pub(crate) ";
            self.result += memb_var_ident.0;
            self.result += ": ";
            match memb_var_signature.contract {
                MembVarContract::Own => (),
                MembVarContract::Ref => todo!(),
            }
            self.gen_scope(memb_var_signature.ty);
            self.result += ",\n";
        }
        self.result += "}\n";
        // impl member routines
        if memb_routines.len() > 0 {
            self.write("\nimpl ");
            self.write(&tyname);
            self.write(" {\n");
            for (memb_routine_ident, memb_routine) in memb_routines {
                self.write("    pub(crate) fn ");
                self.write(&memb_routine_ident);
                self.write("(");
                match memb_routine.this_contract {
                    InputContract::Pure => self.write("&self"),
                    InputContract::GlobalRef => todo!(),
                    InputContract::Take => todo!(),
                    InputContract::BorrowMut => todo!(),
                    InputContract::TakeMut => todo!(),
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
                        InputContract::Take => todo!(),
                        InputContract::BorrowMut => todo!(),
                        InputContract::TakeMut => todo!(),
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
            self.write("}\n");
        }
    }
}
