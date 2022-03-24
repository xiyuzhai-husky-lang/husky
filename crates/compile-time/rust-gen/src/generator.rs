mod impl_expr;
mod impl_routine;
mod impl_scope;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use semantics_entity::{Entity, EntityKind, TyKind};
use semantics_package::Package;
use std::sync::Arc;
use word::WordInterner;

pub(crate) struct RustGenerator<'a> {
    db: &'a dyn RustGenQueryGroup,
    indent: fold::Indent,
    result: String,
}

impl<'a> RustGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustGenQueryGroup) -> Self {
        Self {
            db,
            indent: 0,
            result: Default::default(),
        }
    }

    pub(crate) fn gen_package_lib_rs(&mut self, package: &Package) {
        for entity in package.subentities.iter() {
            match entity.kind() {
                EntityKind::Main(_) => todo!(),
                EntityKind::Module {} => todo!(),
                EntityKind::Feature(_) => todo!(),
                EntityKind::Pattern {} => todo!(),
                EntityKind::Func {
                    input_placeholders,
                    output,
                    stmts,
                } => self.gen_func_defn(entity.ident, input_placeholders, output.scope, stmts),
                EntityKind::Proc {
                    input_placeholders,
                    output,
                    stmts,
                } => self.gen_proc_defn(entity.ident, input_placeholders, output.scope, stmts),
                EntityKind::Ty(ty) => match ty.kind {
                    TyKind::Enum { ref variants } => self.gen_enum_defn(entity.ident, variants),
                    TyKind::Struct {
                        ref memb_vars,
                        ref memb_routines,
                    } => self.gen_struct_defn(entity.ident, memb_vars, memb_routines),
                },
            }
        }
        self.gen_init(&package.subentities);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    fn gen_init(&mut self, subentities: &[Arc<Entity>]) {
        msg_once!("link entity with compiled");
        self.result += r#"
pub mod __init__ {
    pub fn link_entity_with_compiled(compile_time: &mut compile_time_db::HuskyLangCompileTime) {
"#;
        self.result += "        todo!()\n";
        for entity in subentities {}
        self.result += "    }\n}\n";
    }
}
