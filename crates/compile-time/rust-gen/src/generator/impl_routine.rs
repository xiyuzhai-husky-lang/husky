use entity_route::EntityRoutePtr;
use semantics_eager::{FuncStmt, ProcStmt};
use vm::InputLiason;
use word::CustomIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_proc_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputParameter],
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
        self.gen_entity_route(output);
        self.write(" {\n");
        self.gen_proc_stmts(stmts, 4);
        self.write("}\n");
    }

    pub(super) fn gen_func_defn(
        &mut self,
        ident: CustomIdentifier,
        input_placeholders: &[InputParameter],
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
        self.gen_entity_route(output);
        self.write(" {\n");
        self.gen_func_stmts(stmts, 4);
        self.write("}\n");
    }
}
