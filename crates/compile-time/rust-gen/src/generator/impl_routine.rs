use entity_route::{EntityRoutePtr, InputPlaceholder};
use semantics_eager::{FuncStmt, ProcStmt};
use vm::InputContract;
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_proc_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputPlaceholder],
        output: EntityRoutePtr,
        stmts: &[Arc<ProcStmt>],
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
        self.gen_proc_stmts(stmts, 4);
        self.write("}\n");
    }

    pub(super) fn gen_func_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputPlaceholder],
        output: EntityRoutePtr,
        stmts: &[Arc<FuncStmt>],
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
        self.gen_func_stmts(stmts, 4);
        self.write("}\n");
    }
}
