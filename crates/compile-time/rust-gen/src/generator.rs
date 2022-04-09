mod impl_expr;
mod impl_routine;
mod impl_scope;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use pack_semantics::Pack;
use semantics_entity::{EntityDefn, EntityDefnVariant, TyDefnKind};
use std::sync::Arc;

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

    pub(crate) fn gen_pack_lib_rs(&mut self, pack: &Pack) {
        for entity in pack.subentity_defns.iter() {
            match entity.kind() {
                EntityDefnVariant::Main(_) => todo!(),
                EntityDefnVariant::Module {} => todo!(),
                EntityDefnVariant::Feature { .. } | EntityDefnVariant::Pattern {} => (),
                EntityDefnVariant::Func {
                    input_placeholders,
                    output,
                    stmts,
                } => self.gen_func_defn(
                    entity.ident.custom(),
                    input_placeholders,
                    output.route,
                    stmts,
                ),
                EntityDefnVariant::Proc {
                    input_placeholders,
                    output,
                    stmts,
                } => self.gen_proc_defn(
                    entity.ident.custom(),
                    input_placeholders,
                    output.route,
                    stmts,
                ),
                EntityDefnVariant::Ty(ty) => match ty.kind {
                    TyDefnKind::Enum => self.gen_enum_defn(entity.ident.custom(), &ty.variants),
                    TyDefnKind::Struct => {
                        self.gen_struct_defn(entity.ident.custom(), &ty.fields, &ty.methods)
                    }
                    TyDefnKind::Record { .. } => (),
                },
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::EnumVariant(_) => todo!(),
            }
        }
        self.gen_init(&pack.subentity_defns);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    fn gen_init(&mut self, subentities: &[Arc<EntityDefn>]) {
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
