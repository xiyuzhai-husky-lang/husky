use entity_route::EntityRoutePtr;
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
            self.write(&input_placeholder.ident.ident);
            self.write(": ");
            match input_placeholder.contract {
                InputContract::Pure => {
                    if !self
                        .db
                        .is_copy_constructible(input_placeholder.ranged_ty.route)
                    {
                        self.write("&")
                    }
                }
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
                InputContract::MemberAccess => todo!(),
            }
            self.gen_entity_route(input_placeholder.ranged_ty.route);
        }
        self.write(") -> ");
        self.gen_entity_route(output);
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
            self.write(&input_placeholder.ident.ident);
            self.write(": ");
            match input_placeholder.contract {
                InputContract::Pure => {
                    if !self
                        .db
                        .is_copy_constructible(input_placeholder.ranged_ty.route)
                    {
                        self.write("&")
                    }
                }
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
                InputContract::MemberAccess => todo!(),
            }
            self.gen_entity_route(input_placeholder.ranged_ty.route);
        }
        self.write(") -> ");
        self.gen_entity_route(output);
        self.write(" {\n");
        self.gen_func_stmts(stmts, 4);
        self.write("}\n");
    }
}
