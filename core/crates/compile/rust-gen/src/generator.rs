mod impl_expr;
mod impl_routine;
mod impl_scope;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use entity_kind::TyKind;
use pack_semantics::Package;
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

    pub(crate) fn gen_pack_lib_rs(&mut self, pack: &Package) {
        for entity in pack.subentity_defns.iter() {
            match entity.variant {
                EntityDefnVariant::Main(_) => panic!(),
                EntityDefnVariant::Module { .. } => {
                    self.write("mod ");
                    self.write(&entity.ident);
                    self.write(";\n");
                }
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
                EntityDefnVariant::Ty {
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
                    _ => panic!(),
                },
                EntityDefnVariant::Trait { .. } => todo!(),
                EntityDefnVariant::TraitAssociatedTypeImpl { .. }
                | EntityDefnVariant::TraitAssociatedConstSizeImpl { .. }
                | EntityDefnVariant::Method { .. }
                | EntityDefnVariant::Builtin
                | EntityDefnVariant::EnumVariant { .. }
                | EntityDefnVariant::TyField { .. } => {
                    panic!()
                }
                EntityDefnVariant::Function {
                    ref spatial_parameters,
                    ref parameters,
                    output,
                    ref source,
                } => todo!(),
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
    pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyLangCompileTime) {
"#;
        self.result += "        todo!()\n";
        for entity in subentities {}
        self.result += "    }\n}\n";
    }
}
