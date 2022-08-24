mod impl_resolved_linkage;
mod impl_ty_linkage_entries;

use husky_entity_route::CanonicalEntityRoutePtrKind;
use husky_entity_semantics::{DefinitionRepr, FieldDefnVariant, MethodDefnKind};
use husky_layout::RegMemoryKind;
use husky_word::RootIdentifier;
use infer_decl::{
    CallFormDecl, OutputDecl, ParameterDecl, TraitMemberImplDecl, TyDecl, VariadicTemplate,
};

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_init_content(&mut self) {
        self.write(
            r#"use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &["#,
        );
        let main_module = self.db.module(self.target_entrance).unwrap();
        let entity_link_dependees = self.db.entity_link_dependees(main_module);
        for entity_route in entity_link_dependees.iter() {
            if !entity_route.contains_any() {
                let entity_defn = self.db.entity_defn(*entity_route).unwrap();
                self.gen_linkage_entry(*entity_route, &entity_defn);
            }
        }
        self.write(
            r#"
];
"#,
        );
    }

    fn gen_linkage_entry(&mut self, entity_route: EntityRoutePtr, entity_defn: &EntityDefn) {
        if self.db.is_defn_static(entity_route)
            && !self.db.contains_spatial_parameters(entity_route)
        {
            return;
        }
        match entity_defn.variant {
            EntityDefnVariant::Module { .. } => (),
            EntityDefnVariant::Feature { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { ref expr } => (),
                DefinitionRepr::LazyBlock { ref stmts, ty } => (),
                DefinitionRepr::FuncBlock {
                    route,
                    return_ty: output_ty,
                    ..
                } => self.gen_eager_feature_linkage_entry(route, output_ty.route),
                DefinitionRepr::ProcBlock { .. } => todo!(),
            },
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
            EntityDefnVariant::Method { .. } => {
                self.gen_method_linkage_entry(entity_route);
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_transfer_linkage(
                    self.db.needs_eval_context(entity_route),
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::StaticCallRoute),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Proc { .. } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_transfer_linkage(
                    self.db.needs_eval_context(entity_route),
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::StaticCallRoute),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Ty {
                ty_kind,
                ref members,
                ref opt_type_call,
                ..
            } => match ty_kind {
                TyKind::Record => (),
                _ => self.gen_ty_linkages(ty_kind, opt_type_call, entity_route, members),
            },
            EntityDefnVariant::Trait {
                ref spatial_parameters,
                ref members,
            } => todo!(),
            EntityDefnVariant::EnumVariant { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField { .. } => (), // this is handled in ty defn
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {}
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        }
    }

    fn gen_method_linkage_entry(&mut self, entity_route: EntityRoutePtr) {
        self.write(
            r#"
    ("#,
        );
        match entity_route.variant {
            EntityRouteVariant::Child { parent, ident } => {
                self.write(&format!(
                    r#"
        __StaticLinkageKey::Routine {{ route: "{entity_route}" }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                let this_liason = call_form_decl.this_liason();
                match this_liason {
                    ParameterModifier::MemberAccess => {
                        self.write(&format!(
                            r#"
        method_elem_linkage!("#
                        ));
                        self.gen_entity_route(entity_route.parent(), EntityRouteRole::Decl);
                        let method_name = entity_route.ident().as_str();
                        let mangled_intrinsic_ty_vtable =
                            self.db.mangled_intrinsic_ty_vtable(parent);
                        let mangled_output_ty_vtable = self
                            .db
                            .mangled_intrinsic_ty_vtable(call_form_decl.output.ty());
                        self.write(&format!(
                            ", __registration__::{mangled_intrinsic_ty_vtable}, __registration__::{mangled_output_ty_vtable}, {method_name})"
                        ))
                    }
                    _ => {
                        let method_name_extra =
                            if entity_route.ident().as_str() == "pop_with_largest_opt_f32" {
                                let elem_ty = entity_route.parent().entity_route_argument(0);
                                if self.db.is_copyable(elem_ty).unwrap() {
                                    "_copyable"
                                } else {
                                    "_borrow"
                                }
                            } else {
                                ""
                            };
                        self.gen_transfer_linkage(
                            self.db.needs_eval_context(entity_route),
                            Some((this_liason, entity_route.parent())),
                            |this| {
                                this.write(&format!("__this.{}", entity_route.ident().as_str()));
                                // ad hoc
                                this.write(method_name_extra)
                            },
                            |this| {
                                this.gen_entity_route(
                                    entity_route,
                                    EntityRouteRole::StaticCallRoute,
                                )
                            },
                            &call_form_decl,
                        )
                    }
                }
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                if trai.variant == self.db.entity_route_menu().std_ops_index_trai.variant {
                    let this_ty_decl = self.db.ty_decl(ty).unwrap();
                    let trai_impl = this_ty_decl.trait_impl(trai).unwrap();
                    let elem_ty = match trai_impl.member_impls()[0] {
                        TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
                        _ => panic!(),
                    };
                    self.gen_index_linkage(ty, elem_ty)
                } else {
                    todo!()
                }
            }
            EntityRouteVariant::Root { ident } => todo!(),
            EntityRouteVariant::Package { main, ident } => todo!(),
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::Any {
                ident,
                husky_entity_kind,
                ..
            } => todo!(),
            EntityRouteVariant::ThisType { .. } => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
        }
        self.write(
            r#"
    ),"#,
        );
    }

    fn gen_eager_feature_linkage_entry(
        &mut self,
        route: EntityRoutePtr,
        output_ty: EntityRoutePtr,
    ) {
        self.write(&format!(
            r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}"
        }},
        {}feature_linkage!("#,
            match output_ty.is_option() {
                true => "opt_",
                false => "",
            }
        ));
        self.gen_entity_route(route, EntityRouteRole::Caller);
        self.write(", ");
        self.gen_entity_route(output_ty.intrinsic(), EntityRouteRole::Decl);
        self.write(", __registration__::");
        let entity_defn = self.db.entity_defn(route).unwrap();
        self.write(&self.db.mangled_intrinsic_ty_vtable(output_ty));
        self.write(
            r#"),
    ),"#,
        );
    }

    fn gen_index_linkage(&mut self, ty: EntityRoutePtr, elem_ty: EntityRoutePtr) {
        msg_once!("todo: generic indexing");
        // ad hoc
        let mutability = match ty.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootIdentifier::B32 => todo!(),
                RootIdentifier::B64 => todo!(),
                RootIdentifier::Vec => "mutable",
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Array => "mutable",
                _ => panic!(),
            },
            _ => {
                let route_menu = self.db.entity_route_menu();
                if ty.variant == route_menu.std_slice_cyclic_slice.variant {
                    "immutable"
                } else {
                    todo!()
                }
            }
        };
        let canonical_elem_ty = elem_ty.canonicalize();
        self.write(&format!(
            r#"
        __StaticLinkageKey::Index {{
            opd_tys: &["{ty:?}", "i32"],
        }},"#,
        ));
        let canonical_elem_ty_kind = canonical_elem_ty.kind();
        let elem_ty_reg_memory_kind = self.db.reg_memory_kind(elem_ty);
        self.write(&format!(
            r#"
        index_linkage!(
            {mutability},
            {canonical_elem_ty_kind},
            {elem_ty_reg_memory_kind},"#
        ));
        let mangled_intrinsic_ty_vtable = self.db.mangled_intrinsic_ty_vtable(ty);
        let mangled_intrinsic_elem_ty_vtable = self.db.mangled_intrinsic_ty_vtable(elem_ty);
        let intrinsic_elem_ty = elem_ty.intrinsic();
        self.gen_entity_route(ty, EntityRouteRole::Decl);
        self.write(format!(
            r#",
            __registration__::{mangled_intrinsic_ty_vtable},
            "#
        ));
        self.gen_entity_route(intrinsic_elem_ty, EntityRouteRole::Decl);
        self.write(format!(
            r#",
            __registration__::{mangled_intrinsic_elem_ty_vtable}
)"#
        ))
    }

    fn gen_parameter_downcast(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty();
        match parameter.liason {
            ParameterModifier::None => {
                if parameter_ty.is_eval_ref() {
                    self.gen_parameter_downcast_eval_ref(i, parameter)
                } else {
                    if self.db.is_copyable(parameter_ty).unwrap() {
                        self.gen_parameter_downcast_copy(i, parameter)
                    } else {
                        self.gen_parameter_downcast_temp_ref(i, parameter)
                    }
                }
            }
            ParameterModifier::Move => self.gen_parameter_downcast_move(i, parameter),
            ParameterModifier::MoveMut => todo!(),
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::EvalRef => self.gen_parameter_downcast_eval_ref(i, parameter),
            ParameterModifier::TempRef => todo!(),
            ParameterModifier::TempRefMut => todo!(),
        }
    }

    fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty();
        self.write(&format!(
            r#"
                    let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        match parameter_ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::I64
                | RootIdentifier::F32
                | RootIdentifier::F64
                | RootIdentifier::B32
                | RootIdentifier::B64 => self.write(&format!(
                    " = __arguments[{i}].downcast_{root_identifier}();"
                )),
                RootIdentifier::Bool => todo!(),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::ThickFp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::RefMut => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                let parameter_ty_decl: Arc<TyDecl> = self.db.ty_decl(parameter_ty).unwrap();
                match parameter_ty_decl.ty_kind {
                    TyKind::Enum => self.write(&format!(
                        " = __arguments[{i}].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();"
                    )),
                    TyKind::Record => todo!(),
                    TyKind::Struct => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Slice => todo!(),
                    TyKind::CyclicSlice => todo!(),
                    TyKind::Tuple => todo!(),
                    TyKind::Mor => todo!(),
                    TyKind::ThickFp =>{
                        self.write(&format!(
                        r#" = unsafe {{ __arguments[{i}]
                        .downcast_temp_ref::<__VirtualFunction>(&__registration__::__VIRTUAL_FUNCTION_VTABLE)
                        .downcast_thick_fp() }};"#
                        ));
                    },
                    TyKind::AssociatedAny => todo!(),
                    TyKind::ThisAny => todo!(),
                    TyKind::SpatialPlaceholderAny => todo!(),
                    TyKind::BoxAny => todo!(),
                    TyKind::HigherKind => todo!(),
                    TyKind::Ref => todo!(),
                    TyKind::Option => todo!(),
                    TyKind::TargetOutputAny => todo!(),
                }
            }
        }
    }

    fn gen_parameter_downcast_move(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty();
        self.write(&format!(
            r#"
                    let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
        self.write(&format!(
            " = unsafe {{ __arb_ref(&__arguments[{i}]) }}.downcast_move(&__registration__::{mangled_parameter_ty_vtable});"
        ))
    }

    fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty();
        self.write(&format!(
            r#"
                    let {parameter_name}: &"#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
        self.write(&format!(
            " = __arguments[{i}].downcast_temp_ref(&__registration__::{mangled_parameter_ty_vtable});"
        ))
    }

    fn gen_parameter_downcast_eval_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty();
        self.write(&format!(
            r#"
                    let {parameter_name}: &'eval "#
        ));
        let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
        self.gen_entity_route(parameter_ty.intrinsic(), EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_eval_ref(&__registration__::{mangled_parameter_ty_vtable});"))
    }
}
