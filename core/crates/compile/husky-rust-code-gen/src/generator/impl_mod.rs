use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_mod_rs_content(&mut self, subentities: &[Arc<EntityDefn>]) {
        for entity in subentities.iter() {
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
                } => self.gen_func_defn(entity.base_route, parameters, output.route, stmts),
                EntityDefnVariant::Proc {
                    parameters: ref parameters,
                    output,
                    ref stmts,
                    ..
                } => self.gen_proc_defn(entity.base_route, parameters, output.route, stmts),
                EntityDefnVariant::Ty {
                    ref ty_members,
                    ref variants,
                    kind,
                    ref trait_impls,
                    ref members,
                    ..
                } => match kind {
                    TyKind::Enum => self.gen_enum_defn(entity.ident.custom(), variants),
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
                EntityDefnVariant::Function {
                    ref spatial_parameters,
                    ref parameters,
                    output,
                    ref source,
                } => todo!(),
            }
        }
    }
}
