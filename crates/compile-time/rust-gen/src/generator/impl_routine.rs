use scope::{InputPlaceholder, ScopePtr};
use semantics_eager::{DeclStmt, ImprStmt};
use vm::InputContract;
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_proc_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputPlaceholder],
        output: ScopePtr,
        stmts: &[Arc<ImprStmt>],
    ) {
        self.write("\npub(crate) fn ");
        self.write(&ident);
        self.write("(");
        for (i, input_placeholder) in input_placeholders.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
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
        self.gen_scope(output);
        self.write(" {\n");
        self.gen_impr_stmts(stmts, 4);
        self.write("}\n");
    }

    pub(super) fn gen_func_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputPlaceholder],
        output: ScopePtr,
        stmts: &[Arc<DeclStmt>],
    ) {
        self.write("\npub(crate) fn ");
        self.write(&ident);
        self.write("(");
        for (i, input_placeholder) in input_placeholders.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
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
        self.gen_scope(output);
        self.write(" {\n");
        self.gen_decl_stmts(stmts, 4);
        self.write("}\n");
    }
}
