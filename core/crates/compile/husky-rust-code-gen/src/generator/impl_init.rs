use husky_entity_semantics::{DefinitionRepr, FieldDefnVariant};

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_init_content(&mut self) {
        self.write(
            r#"
use crate::*;
use __husky_root::__init_utils::*;

pub fn link_entity_with_compiled(compile_time: &mut husky_compile_time::HuskyCompileTime) {
"#,
        );
        let main_module = self.db.module(self.package_main).unwrap();
        let entity_dependees = self.db.entity_dependees(main_module).unwrap();
        self.write("    compile_time.load_linkages(&[");
        for (entity_route, _) in entity_dependees.iter() {
            let entity_defn = self.db.entity_defn(*entity_route).unwrap();
            self.gen_linkage_entry(*entity_route, &entity_defn);
        }
        self.write("\n    ])\n}\n");
    }

    fn gen_linkage_entry(&mut self, entity_route: EntityRoutePtr, entity_defn: &EntityDefn) {
        if self.db.is_defn_static(entity_route)
            || !self.db.contains_spatial_parameters(entity_route)
        {
            return;
        }
        match entity_defn.variant {
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Module { ref module_items } => (),
            EntityDefnVariant::Feature { ty, ref defn_repr } => match defn_repr {
                DefinitionRepr::LazyExpr { expr } => (),
                DefinitionRepr::LazyBlock { stmts, ty } => (),
                DefinitionRepr::FuncBlock {
                    route,
                    file,
                    range,
                    stmts,
                    ty,
                } => todo!(),
                DefinitionRepr::ProcBlock {
                    file,
                    range,
                    stmts,
                    ty,
                } => todo!(),
            },
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
                self.write("\n    (\n");
                self.write(&format!(
                    r#"        __StaticLinkageKey::Routine {{
            routine: "{entity_route}"
        }},
"#,
                ));
                let nargs = parameters.len() + 1;
                self.write(&format!(
                    "        specific_transfer_linkage!(|_|todo!(), {nargs}),"
                ));
                self.write("\n    ),");
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.write("\n    (\n");
                self.write(&format!(
                    r#"        __StaticLinkageKey::Routine {{
            routine: "{}"
        }},
"#,
                    entity_route
                ));
                let nargs = parameters.len();
                self.write(&format!(
                    "        specific_transfer_linkage!(|_|todo!(), {nargs}),"
                ));
                self.write("\n    ),");
            }
            EntityDefnVariant::Proc {
                ref generic_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.write("\n    (\n");
                self.write(&format!(
                    r#"        __StaticLinkageKey::Routine {{
            routine: "{}"
        }},"#,
                    entity_route
                ));
                let nargs = parameters.len();
                self.write(&format!(
                    r#"
        specific_transfer_linkage!({{
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __EvalResult<__TempValue<'temp, 'eval>> {{"#
                ));
                for (i, parameter) in parameters.iter().enumerate() {
                    self.gen_parameter_downcast(i, parameter)
                }
                self.write(
                    r#"
                Ok(__TempValue::OwnedEval(__OwnedValue::new(
                    "#,
                );
                self.gen_entity_route(entity_route, EntityRouteRole::Caller);
                self.write("(");
                for (i, parameter) in parameters.iter().enumerate() {
                    if i > 0 {
                        self.write(", ")
                    }
                    self.write(&parameter.ranged_ident.ident)
                }
                self.write(
                    r#")
                )))"#,
                );
                self.write(&format!(
                    r#"
            }}
            __wrapper
        }}, {nargs}),
"#
                ));
                self.write("    ),");
            }
            EntityDefnVariant::Ty {
                ref generic_parameters,
                ref ty_members,
                ref variants,
                kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                ref opt_visualizer_source,
            } => {
                if let Some(type_call) = opt_type_call {
                    self.write("\n    (\n");
                    self.write(&format!(
                        r#"        __StaticLinkageKey::TypeCall {{
            ty: "{}"
        }},
"#,
                        entity_route
                    ));
                    let nargs = type_call.parameters.len();
                    self.write(&format!(
                        "        specific_transfer_linkage!(|_|todo!(), {nargs}),"
                    ));
                    self.write("\n    ),");
                }
                for ty_member in members.iter() {
                    match ty_member.variant {
                        EntityDefnVariant::TyField {
                            ty,
                            ref field_variant,
                            liason,
                            opt_linkage,
                        } => match field_variant {
                            FieldDefnVariant::StructOriginal
                            | FieldDefnVariant::StructDefault { .. }
                            | FieldDefnVariant::StructDerivedEager { .. } => {
                                self.write("\n    (\n");
                                let field_ident = ty_member.ident.as_str();
                                self.write(&format!(
                                    r#"        __StaticLinkageKey::StructFieldAccess {{
            this_ty: "{entity_route}",
            field_ident: "{field_ident}",
        }},
        field_linkage!("#,
                                ));
                                self.gen_entity_route(entity_route, EntityRouteRole::Decl);
                                self.write(", ");
                                self.write(field_ident);
                                self.write(")\n    ),");
                            }
                            FieldDefnVariant::StructDerivedLazy { defn_repr } => (),
                            FieldDefnVariant::RecordOriginal
                            | FieldDefnVariant::RecordDerived { .. } => {
                                panic!()
                            }
                        },
                        _ => {
                            if self.db.is_defn_static(entity_route) {
                                p!(entity_route);
                                todo!()
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            EntityDefnVariant::Trait {
                ref generic_parameters,
                ref members,
            } => todo!(),
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => panic!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {
                p!(entity_route);
                p!(trai, ty);
                todo!()
            }
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        }
    }

    fn gen_parameter_downcast(&mut self, i: usize, parameter: &Parameter) {
        let parameter_name = parameter.ranged_ident.ident;
        let parameter_ty = parameter.ranged_ty.route;
        match parameter.ranged_liason.liason {
            ParameterLiason::Pure => {
                if parameter_ty.is_ref() {
                    todo!()
                } else {
                    if self.db.is_copyable(parameter_ty).unwrap() {
                        self.gen_parameter_downcast_copy(i, parameter)
                    } else {
                        self.gen_parameter_downcast_temp_ref(i, parameter)
                    }
                }
            }
            ParameterLiason::Move => todo!(),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::EvalRef => self.gen_parameter_downcast_eval_ref(i, parameter),
            ParameterLiason::TempRef => todo!(),
            ParameterLiason::TempRefMut => todo!(),
        }
    }

    fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &Parameter) {
        let parameter_name = parameter.ranged_ident.ident;
        let parameter_ty = parameter.ranged_ty.route;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_copy();"))
    }

    fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &Parameter) {
        let parameter_name = parameter.ranged_ident.ident;
        let parameter_ty = parameter.ranged_ty.route;
        self.write(&format!(
            r#"
                let {parameter_name}: &"#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_temp_ref();"))
    }

    fn gen_parameter_downcast_eval_ref(&mut self, i: usize, parameter: &Parameter) {
        let parameter_name = parameter.ranged_ident.ident;
        let parameter_ty = parameter.ranged_ty.route;
        self.write(&format!(
            r#"
                let {parameter_name}: &'eval "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_eval_ref();"))
    }
}
