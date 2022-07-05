use husky_entity_route::entity_route_menu;
use husky_entity_route::{make_subroute, make_type_as_trait_member_route};
use husky_entity_semantics::{DefinitionRepr, FieldDefnVariant, MethodDefnKind};
use infer_decl::{OutputDecl, ParameterDecl};

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
            && !self.db.contains_spatial_parameters(entity_route)
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
                method_defn_kind, ..
            } => {
                self.write("\n    (\n");
                match method_defn_kind {
                    MethodDefnKind::TypeMethod { .. } => {
                        self.write(&format!(
                            r#"        __StaticLinkageKey::Routine {{
            routine: "{entity_route}"
        }},"#,
                        ));
                        let method_decl = self.db.method_decl(entity_route).unwrap();
                        match method_decl.this_liason {
                            ParameterLiason::MemberAccess => {
                                self.write(&format!(
                                    r#"
        method_elem_linkage!("#
                                ));
                                self.gen_entity_route(entity_route.parent(), EntityRouteRole::Decl);
                                let method_name = entity_route.ident().as_str();
                                self.write(&format!(", {method_name})"))
                            }
                            _ => self.gen_specific_routine_linkage(
                                Some((method_decl.this_liason, entity_route.parent())),
                                |this| {
                                    this.write(&format!(
                                        "__this.{}",
                                        entity_route.ident().as_str()
                                    ));
                                },
                                &method_decl.parameters,
                                &method_decl.output,
                            ),
                        }
                    }
                    MethodDefnKind::TraitMethod { trai } => todo!(),
                    MethodDefnKind::TraitMethodImpl { trai } => {
                        if trai.kind == entity_route_menu().std_ops_index_trai.kind {
                            match entity_route.kind {
                                EntityRouteKind::Root { ident } => todo!(),
                                EntityRouteKind::Package { main, ident } => todo!(),
                                EntityRouteKind::Child { parent, ident } => todo!(),
                                EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                                    self.gen_index_linkage(ty)
                                }
                                EntityRouteKind::Input { main } => todo!(),
                                EntityRouteKind::Generic { ident, entity_kind } => todo!(),
                                EntityRouteKind::ThisType => todo!(),
                            }
                        } else {
                            todo!()
                        }
                    }
                }
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
            EntityDefnVariant::Proc { .. } => {
                self.write("\n    (\n");
                self.write(&format!(
                    r#"        __StaticLinkageKey::Routine {{
            routine: "{}"
        }},"#,
                    entity_route
                ));
                let function_decl = self.db.function_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_specific_routine_linkage(
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    &function_decl.primary_parameters,
                    &function_decl.output,
                );
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
                ref opt_visual_stmts,
                ..
            } => {
                if let Some(_) = opt_type_call {
                    self.write("\n    (\n");
                    self.write(&format!(
                        r#"        __StaticLinkageKey::TypeCall {{
            ty: "{}"
        }},
"#,
                        entity_route
                    ));
                    let function_decl = self.db.function_decl(entity_route).unwrap();
                    msg_once!("keyword parameters");
                    self.gen_specific_routine_linkage(
                        None,
                        |this| {
                            this.gen_entity_route(entity_route, EntityRouteRole::Caller);
                            this.write("::__call__")
                        },
                        &function_decl.primary_parameters,
                        &function_decl.output,
                    );
                    self.write("\n    ),");
                }
                for ty_member in members.iter() {
                    let is_defn_static = self.db.is_defn_static(entity_route);
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
        {}!("#,
                                    match liason {
                                        MemberLiason::Immutable => "field_linkage",
                                        MemberLiason::Mutable => {
                                            if ty.is_ref() {
                                                todo!()
                                            } else {
                                                "mut_field_linkage"
                                            }
                                        }
                                        MemberLiason::Derived => todo!(),
                                    }
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
                            if is_defn_static {
                                let member_entity_route = match ty_member.base_route.kind {
                                    EntityRouteKind::Root { ident } => {
                                        p!(ty_member.base_route, ty_member.ident);
                                        todo!()
                                    }
                                    EntityRouteKind::Package { main, ident } => todo!(),
                                    EntityRouteKind::Child { parent, ident } => make_subroute(
                                        entity_route,
                                        ty_member.ident.custom(),
                                        Default::default(),
                                    ),
                                    EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                                        msg_once!("todo: ignore trait that is generic over a specific type");
                                        make_type_as_trait_member_route(
                                            entity_route,
                                            trai,
                                            ident,
                                            Default::default(),
                                        )
                                    }
                                    EntityRouteKind::Input { main } => todo!(),
                                    EntityRouteKind::Generic { ident, entity_kind } => todo!(),
                                    EntityRouteKind::ThisType => todo!(),
                                };
                                self.gen_linkage_entry(member_entity_route, ty_member)
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
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {}
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        }
    }

    fn gen_specific_routine_linkage(
        &mut self,
        opt_this: Option<(ParameterLiason, EntityRoutePtr)>,
        gen_caller: impl FnOnce(&mut Self),
        parameters: &[ParameterDecl],
        output: &OutputDecl,
    ) {
        let base = if opt_this.is_some() { 1 } else { 0 };
        let nargs = parameters.len() + base;
        self.write(&format!(
            r#"
        specific_transfer_linkage!({{
            fn __wrapper<'temp, 'eval>(
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {{"#
        ));
        if let Some((this_liason, this_ty)) = opt_this {
            match this_liason {
                ParameterLiason::Pure => {
                    self.write(&format!(
                        r#"
                let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&");
                        self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_temp_ref();"))
                    }
                }
                ParameterLiason::Move => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => panic!(),
                ParameterLiason::EvalRef => {
                    self.write(&format!(
                        r#"
                let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&'eval ");
                        self.gen_entity_route(this_ty.deref_route(), EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_eval_ref();"))
                    }
                }
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => {
                    self.write(&format!(
                        r#"
                let __this: "#
                    ));
                    self.write("&mut ");
                    self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                    self.write(&format!(
                        " = unsafe {{ __arb_ref(&__arguments[0]) }}.downcast_mut();"
                    ))
                }
            }
        }
        for (i, parameter) in parameters.iter().enumerate() {
            self.gen_parameter_downcast(i + base, parameter)
        }
        if self.db.is_copyable(output.ty).unwrap() {
            self.write(
                r#"
                __TempValue::Copyable(
                    "#,
            );
        } else {
            self.write(
                r#"
                __TempValue::OwnedEval(__OwnedValue::new(
                    "#,
            );
        }
        gen_caller(self);
        self.write("(");
        for (i, parameter) in parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&parameter.ident)
        }
        if self.db.is_copyable(output.ty).unwrap() {
            self.write(
                r#")
                    .__take_copyable_dyn())"#,
            );
        } else {
            self.write(
                r#")
                    ))"#,
            );
        }
        self.write(&format!(
            r#"
            }}
            __wrapper
        }}, {nargs}),
"#
        ));
    }

    fn gen_index_linkage(&mut self, ty: EntityRoutePtr) {
        msg_once!("todo: generic indexing");
        self.write(&format!(
            r#"
        __StaticLinkageKey::Index {{
            opd_tys: &["{ty:?}", "i32"],
        }},"#,
        ));
        let nargs = 2;
        self.write(&format!(
            r#"
        index_linkage!("#
        ));
        self.gen_entity_route(ty, EntityRouteRole::Decl);
        self.write(")")
    }

    fn gen_parameter_downcast(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        match parameter.liason {
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
            ParameterLiason::Move => self.gen_parameter_downcast_move(i, parameter),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::EvalRef => self.gen_parameter_downcast_eval_ref(i, parameter),
            ParameterLiason::TempRef => todo!(),
            ParameterLiason::TempRefMut => todo!(),
        }
    }

    fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_copy();"))
    }

    fn gen_parameter_downcast_move(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(
            " = unsafe {{ __arb_ref(&__arguments[{i}]) }}.downcast_move();"
        ))
    }

    fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &"#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_temp_ref();"))
    }

    fn gen_parameter_downcast_eval_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &'eval "#
        ));
        self.gen_entity_route(parameter_ty.deref_route(), EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_eval_ref();"))
    }
}
