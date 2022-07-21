use super::*;
use husky_entity_route::entity_route_menu;
use husky_entity_semantics::{
    CallFormSource, DefinitionRepr, EntityDefn, EntityDefnVariant, EnumVariantDefnVariant,
    FieldDefnVariant, MethodDefnKind,
};

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_from_entity_defn(&mut self, defn: &EntityDefn) {
        match defn.variant {
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Module { ref module_items } => module_items
                .iter()
                .for_each(|item| self.collect_from_entity_defn(item)),
            EntityDefnVariant::Feature { ref defn_repr } => {
                self.collect_from_feature_repr(defn.base_route, defn_repr)
            }
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
            EntityDefnVariant::Method {
                ref spatial_parameters,
                this_liason,
                ref parameters,
                output_ty,
                output_liason,
                method_defn_kind,
                ref opt_source,
            } => {
                self.insert(defn.base_route);
                self.insert(defn.base_route.parent());
                self.collect_from_parameters(parameters);
                self.insert(output_ty.route);
                if let Some(source) = opt_source {
                    match source {
                        CallFormSource::Func { stmts } => self.collect_from_func_stmts(stmts),
                        CallFormSource::Proc { stmts } => self.collect_from_proc_stmts(stmts),
                        CallFormSource::Lazy { stmts } => (),
                        CallFormSource::Static(_) => (),
                    }
                }
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.insert(defn.base_route);
                self.collect_from_parameters(parameters);
                self.insert(output.route);
                self.collect_from_func_stmts(stmts)
            }
            EntityDefnVariant::Proc {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.insert(defn.base_route);
                self.collect_from_parameters(parameters);
                self.insert(output.route);
                self.collect_from_proc_stmts(stmts)
            }
            EntityDefnVariant::Ty {
                spatial_parameters: ref generic_parameters,
                ref ty_members,
                ref variants,
                ty_kind: kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                opt_static_visual_ty,
                ref opt_visual_stmts,
            } => {
                if opt_type_call.is_some() {
                    self.insert(defn.base_route)
                }
                let entity_route_menu = entity_route_menu();
                for member in members.iter() {
                    match member.variant {
                        EntityDefnVariant::TyField {
                            ty,
                            ref field_variant,
                            liason,
                            opt_linkage,
                        } => self.insert(ty),
                        EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {
                            if defn.base_route == entity_route_menu.clone_trait {
                                ()
                            } else {
                                self.insert(defn.base_route)
                            }
                        }
                        EntityDefnVariant::Method {
                            method_defn_kind, ..
                        } => match method_defn_kind {
                            MethodDefnKind::TypeMethod { .. } => self.insert(defn.base_route),
                            MethodDefnKind::TraitMethod { trai } => self.insert(defn.base_route),
                            MethodDefnKind::TraitMethodImpl { trai } => {
                                self.insert(defn.base_route)
                            }
                        },
                        _ => self.insert(member.base_route),
                    }
                }
            }
            EntityDefnVariant::Trait {
                spatial_parameters: ref generic_parameters,
                ref members,
            } => {
                p!(defn.base_route);
                todo!()
            }
            EntityDefnVariant::EnumVariant { ident, ref variant } => match variant {
                EnumVariantDefnVariant::Constant => todo!(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {
                todo!()
            }
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => {
                todo!()
            }
        }
    }

    fn collect_from_parameters(&mut self, parameters: &[Parameter]) {
        for parameter in parameters {
            self.insert(parameter.ranged_ty.route)
        }
    }

    fn collect_from_feature_repr(
        &mut self,
        feature_route: EntityRoutePtr,
        feature_repr: &DefinitionRepr,
    ) {
        match feature_repr {
            DefinitionRepr::LazyExpr { expr } => (),
            DefinitionRepr::LazyBlock { stmts, ty } => (),
            DefinitionRepr::FuncBlock {
                route,
                file,
                range,
                stmts,
                ty,
            } => {
                self.insert(feature_route);
                self.insert(ty.route);
                self.collect_from_func_stmts(stmts)
            }
            DefinitionRepr::ProcBlock {
                file,
                range,
                stmts,
                ty,
            } => {
                self.insert(feature_route);
                self.insert(ty.route);
                self.collect_from_proc_stmts(stmts)
            }
        }
    }
}
