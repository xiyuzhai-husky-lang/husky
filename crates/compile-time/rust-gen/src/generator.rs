mod impl_expr;
mod impl_routine;
mod impl_scope;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use entity_kind::TyKind;
use pack_semantics::Pack;
use semantics_entity::{EntityDefn, EntityDefnVariant};
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
            match entity.variant {
                EntityDefnVariant::Main(_) => todo!(),
                EntityDefnVariant::Module { .. } => todo!(),
                EntityDefnVariant::Feature { .. } => (),
                EntityDefnVariant::Func {
                    ref parameters,
                    output,
                    ref stmts,
                    ..
                } => self.gen_func_defn(entity.ident.custom(), parameters, output.route, stmts),
                EntityDefnVariant::Proc {
                    parameters: ref parameters,
                    output,
                    ref stmts,
                    ..
                } => self.gen_proc_defn(entity.ident.custom(), parameters, output.route, stmts),
                EntityDefnVariant::Type {
                    ref ty_members,
                    ref variants,
                    kind,
                    ref trait_impls,
                    ref members,
                    ..
                } => match kind {
                    TyKind::Enum => self.gen_enum_defn(entity.ident.custom(), variants),
                    TyKind::Struct => {
                        self.gen_struct_defn(entity.ident.custom(), ty_members, trait_impls)
                    }
                    TyKind::Record { .. } => (),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                },
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::EnumVariant { .. } => todo!(),
                EntityDefnVariant::TypeField {
                    ty,
                    ref fieldiant,
                    contract,
                } => todo!(),
                EntityDefnVariant::Method { .. } => todo!(),
                EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
                EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
                EntityDefnVariant::Trait { .. } => todo!(),
            }
        }
        self.gen_init(&pack.subentity_defns);
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    fn gen_init(&mut self, subentities: &[Arc<EntityDefn>]) {
        emsg_once!("link entity with compiled");
        self.result += r#"
pub mod __init__ {
    pub fn link_entity_with_compiled(compile_time: &mut compile_time_db::HuskyLangCompileTime) {
"#;
        self.result += "        todo!()\n";
        for entity in subentities {}
        self.result += "    }\n}\n";
    }
}
