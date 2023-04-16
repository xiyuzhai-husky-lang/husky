use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_mod_rs_content(&mut self, subentities: &[Arc<EntityDefn>]) {
        for entity in subentities.iter() {
            match entity.variant {
                EntityDefnVariant::Module { .. } => {
                    self.write("pub(crate) mod ");
                    self.write(&entity.ident);
                    self.write(";\n");
                }
                EntityDefnVariant::Feature { ref defn_repr } => {
                    self.gen_feature_defn(entity.base_route, defn_repr)
                }
                EntityDefnVariant::Func {
                    ref parameters,
                    output,
                    ref stmts,
                    ..
                } => self.gen_func_defn(0, entity.base_route, parameters, output.route, stmts),
                EntityDefnVariant::Proc {
                    ref parameters,
                    output,
                    ref stmts,
                    ..
                } => self.gen_proc_defn(0, entity.base_route, parameters, output.route, stmts),
                EntityDefnVariant::EtherealTerm {
                    ref ty_members,
                    ref variants,
                    ty_kind: kind,
                    ref trait_impls,
                    ..
                } => match kind {
                    TyKind::Enum => {
                        self.gen_enum_defn(entity.base_route, entity.ident.custom(), variants)
                    }
                    TyKind::Struct => self.gen_struct_defn(
                        entity.base_route,
                        entity.ident.custom(),
                        ty_members,
                        trait_impls,
                    ),
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
                EntityDefnVariant::Function { .. } => todo!(),
                EntityDefnVariant::TargetInput { .. } => todo!(),
                EntityDefnVariant::Any => todo!(),
            }
        }
    }
}
