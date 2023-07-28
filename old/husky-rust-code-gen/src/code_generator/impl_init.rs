mod impl_resolved_linkage;
mod impl_ty_linkage_entries;

use husky_item_semantics::{DefinitionRepr, FieldDefnVariant};

use husky_coword::RootBuiltinIdent;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_init_content(&mut self) {
        self.write(
            r#"use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &["#,
        );
        todo!();
        // let main_module = self.db.module(self.target_entrance).unwrap();
        // let item_link_dependees = self.db.item_link_dependees(main_module);
        // for item_path in item_link_dependees.iter() {
        //     if !item_path.contains_any() {
        //         let item_defn = self.db.item_defn(*item_path).unwrap();
        //         self.gen_linkage_entry(*item_path, &item_defn);
        //     }
        // }
        self.write(
            r#"
];
"#,
        );
    }

    fn gen_linkage_entry(&mut self, _item_route: EtherealTerm, _item_defn: &EntityDefn) {
        todo!()
        //     if self.db.is_defn_static(item_path)
        //         && !self.db.contains_spatial_parameters(item_path)
        //     {
        //         return;
        //     }
        //     match item_defn.variant {
        //         EntityDefnVariant::Module { .. } => (),
        //         EntityDefnVariant::Feature { ref defn_repr } => match **defn_repr {
        //             DefinitionRepr::LazyExpr { .. } => (),
        //             DefinitionRepr::LazyBlock { .. } => (),
        //             DefinitionRepr::FuncBlock {
        //                 route, return_ty, ..
        //             }
        //             | DefinitionRepr::ProcBlock {
        //                 route, return_ty, ..
        //             } => self.gen_eager_feature_linkage_entry(route, return_ty.route),
        //         },
        //         EntityDefnVariant::Function { .. } => todo!(),
        //         EntityDefnVariant::Method { .. } => {
        //             self.gen_method_linkage_entry(item_path);
        //         }
        //         EntityDefnVariant::Func { .. } => {
        //             self.write(&format!(
        //                 r#"
        // (
        //     __StaticLinkageKey::Routine {{
        //         route: "{item_path}",
        //     }},"#,
        //             ));
        //             let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(item_path).unwrap();
        //             msg_once!("keyword_parameters");
        //             self.gen_transfer_linkage(
        //                 self.db.needs_eval_context(item_path),
        //                 None,
        //                 |this| this.gen_item_route(item_path, EntityRouteRole::Caller),
        //                 |this| this.gen_item_route(item_path, EntityRouteRole::StaticCallRoute),
        //                 &call_fugitive_syn_decl,
        //             );
        //             self.write(
        //                 r#"
        // ),"#,
        //             );
        //         }
        //         EntityDefnVariant::Proc { .. } => {
        //             self.write(&format!(
        //                 r#"
        // (
        //     __StaticLinkageKey::Routine {{
        //         route: "{item_path}",
        //     }},"#,
        //             ));
        //             let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(item_path).unwrap();
        //             msg_once!("keyword_parameters");
        //             self.gen_transfer_linkage(
        //                 self.db.needs_eval_context(item_path),
        //                 None,
        //                 |this| this.gen_item_route(item_path, EntityRouteRole::Caller),
        //                 |this| this.gen_item_route(item_path, EntityRouteRole::StaticCallRoute),
        //                 &call_fugitive_syn_decl,
        //             );
        //             self.write(
        //                 r#"
        // ),"#,
        //             );
        //         }
        //         EntityDefnVariant::EtherealTerm {
        //             ty_kind,
        //             ref members,
        //             ref opt_type_call,
        //             ..
        //         } => match ty_kind {
        //             TyKind::Record => (),
        //             _ => self.gen_ty_linkages(opt_type_call, item_path, members),
        //         },
        //         EntityDefnVariant::Trait { .. } => todo!(),
        //         EntityDefnVariant::EnumVariant { .. } => todo!(),
        //         EntityDefnVariant::Builtin => todo!(),
        //         EntityDefnVariant::TyField { .. } => (), // this is handled in ty defn
        //         EntityDefnVariant::TraitAssociatedTypeImpl { .. } => {}
        //         EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
        //         EntityDefnVariant::TargetInput { .. } => todo!(),
        //         EntityDefnVariant::Any => todo!(),
        //     }
    }

    fn gen_method_linkage_entry(&mut self, _item_route: EtherealTerm) {
        todo!()
        //     self.write(
        //         r#"
        // ("#,
        //     );
        //     match item_path.variant {
        //         EntityRouteVariant::Child { parent, .. } => {
        //             self.write(&format!(
        //                 r#"
        //     __StaticLinkageKey::Routine {{ route: "{item_path}" }},"#,
        //             ));
        //             let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(item_path).unwrap();
        //             let this_liason = call_fugitive_syn_decl.this_liason();
        //             match this_liason {
        //                 ParameterModifier::MemberAccess => {
        //                     self.write(&format!(
        //                         r#"
        //     method_elem_linkage!("#
        //                     ));
        //                     self.gen_item_route(item_path.parent(), EntityRouteRole::Decl);
        //                     let method_name = item_path.ident().as_str();
        //                     let mangled_intrinsic_ty_vtable =
        //                         self.db.mangled_intrinsic_ty_vtable(parent);
        //                     let mangled_return_ty_vtable = self
        //                         .db
        //                         .mangled_intrinsic_ty_vtable(call_fugitive_syn_decl.output.ty());
        //                     self.write(&format!(
        //                         ", __registration__::{mangled_intrinsic_ty_vtable}, __registration__::{mangled_return_ty_vtable}, {method_name})"
        //                     ))
        //                 }
        //                 _ => {
        //                     let method_name_extra =
        //                         if item_path.ident().as_str() == "pop_with_largest_opt_f32" {
        //                             let elem_ty = item_path.parent().item_route_argument(0);
        //                             if self.db.is_copyable(elem_ty).unwrap() {
        //                                 "_copyable"
        //                             } else {
        //                                 "_borrow"
        //                             }
        //                         } else {
        //                             ""
        //                         };
        //                     self.gen_transfer_linkage(
        //                         self.db.needs_eval_context(item_path),
        //                         Some((this_liason, item_path.parent())),
        //                         |this| {
        //                             this.write(&format!("__this.{}", item_path.ident().as_str()));
        //                             // ad hoc
        //                             this.write(method_name_extra)
        //                         },
        //                         |this| {
        //                             this.gen_item_route(
        //                                 item_path,
        //                                 EntityRouteRole::StaticCallRoute,
        //                             )
        //                         },
        //                         &call_fugitive_syn_decl,
        //                     )
        //                 }
        //             }
        //         }
        //         EntityRouteVariant::TraitForTypeMember { ty, trai, .. } => {
        //             if trai.variant == self.db.item_route_menu().std_ops_index_trai.variant {
        //                 let this_ty_decl = self.db.ty_decl(ty).unwrap();
        //                 let trai_impl = this_ty_decl.trait_impl(trai).unwrap();
        //                 let elem_ty = match trai_impl.member_impls()[0] {
        //                     TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
        //                     _ => unreachable!(),
        //                 };
        //                 self.gen_index_linkage(ty, elem_ty)
        //             } else {
        //                 todo!()
        //             }
        //         }
        //         EntityRouteVariant::Root { .. } => todo!(),
        //         EntityRouteVariant::Package { .. } => todo!(),
        //         EntityRouteVariant::TargetInputValue => todo!(),
        //         EntityRouteVariant::Any { .. } => todo!(),
        //         EntityRouteVariant::ThisType { .. } => todo!(),
        //         EntityRouteVariant::TargetOutputType => todo!(),
        //     }
        //     self.write(
        //         r#"
        // ),"#,
        //     );
    }

    fn gen_eager_feature_linkage_entry(&mut self, item_path: EntityPath, return_ty: EtherealTerm) {
        self.write(&format!(
            r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}"
        }},
        {}feature_linkage!("#,
            match return_ty.is_option() {
                true => "opt_",
                false => "",
            }
        ));
        self.gen_item_route(route, EntityRouteRole::Caller);
        self.write(", ");
        self.gen_item_route(return_ty.intrinsic(), EntityRouteRole::Decl);
        self.write(", __registration__::");
        self.write(&self.db.mangled_intrinsic_ty_vtable(return_ty));
        self.write(
            r#"),
    ),"#,
        );
    }

    fn gen_index_linkage(&mut self, ty: EtherealTerm, elem_ty: EtherealTerm) {
        msg_once!("todo: generic indexing");
        // ad hoc
        let mutability = match ty.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootBuiltinIdent::B32 => todo!(),
                RootBuiltinIdent::B64 => todo!(),
                RootBuiltinIdent::Vec => "mutable",
                RootBuiltinIdent::Tuple => todo!(),
                RootBuiltinIdent::Array => "mutable",
                _ => panic!(),
            },
            _ => {
                let route_menu = self.db.item_route_menu();
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
        self.gen_item_route(ty, EntityRouteRole::Decl);
        self.write(format!(
            r#",
            __registration__::{mangled_intrinsic_ty_vtable},
            "#
        ));
        self.gen_item_route(intrinsic_elem_ty, EntityRouteRole::Decl);
        self.write(format!(
            r#",
            __registration__::{mangled_intrinsic_elem_ty_vtable}
)"#
        ))
    }

    // fn gen_parameter_downcast(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_ty = parameter.ty();
    //     match parameter.modifier {
    //         ParameterModifier::None => {
    //             if parameter_ty.is_leash() {
    //                 self.gen_parameter_downcast_leash(i, parameter)
    //             } else {
    //                 if self.db.is_copyable(parameter_ty).unwrap() {
    //                     self.gen_parameter_downcast_copy(i, parameter)
    //                 } else {
    //                     self.gen_parameter_downcast_temp_ref(i, parameter)
    //                 }
    //             }
    //         }
    //         ParameterModifier::Owned => {
    //             if self.db.is_copyable(parameter_ty).unwrap() {
    //                 let canonical_parameter_ty = parameter_ty.canonicalize();
    //                 match canonical_parameter_ty.qual() {
    //                     CanonicalQualifier::Intrinsic => {
    //                         self.gen_parameter_downcast_copy(i, parameter)
    //                     }
    //                     CanonicalQualifier::Leash => {
    //                         match canonical_parameter_ty.option_level() {
    //                             0 => self.gen_parameter_downcast_leash(i, parameter),
    //                             1 => self.gen_parameter_downcast_opt_leash(i, parameter),
    //                             _ => todo!(),
    //                         }
    //                     }
    //                     CanonicalQualifier::TempRef => todo!(),
    //                     CanonicalQualifier::TempRefMut => todo!(),
    //                 }
    //             } else {
    //                 self.gen_parameter_downcast_move(i, parameter)
    //             }
    //         }
    //         ParameterModifier::OwnedMut => todo!(),
    //         ParameterModifier::MemberAccess => todo!(),
    //         ParameterModifier::Leash => self.gen_parameter_downcast_leash(i, parameter),
    //         ParameterModifier::TempRef => todo!(),
    //         ParameterModifier::TempRefMut => todo!(),
    //     }
    // }

    // fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_name = parameter.ident;
    //     let parameter_ty = parameter.ty();
    //     self.write(&format!(
    //         r#"
    //                 let {parameter_name}: "#
    //     ));
    //     self.gen_item_route(parameter_ty, EntityRouteRole::Decl);
    //     let maybe_opt = match parameter_ty.is_option() {
    //         true => "opt_",
    //         false => "",
    //     };
    //     match parameter_ty.intrinsic() {
    //         EntityRoutePtr::Root(root_identifier) => match root_identifier {
    //             RootBuiltinIdent::Void
    //             | RootBuiltinIdent::I32
    //             | RootBuiltinIdent::I64
    //             | RootBuiltinIdent::F32
    //             | RootBuiltinIdent::F64
    //             | RootBuiltinIdent::B32
    //             | RootBuiltinIdent::B64
    //             | RootBuiltinIdent::Bool => self.write(&format!(
    //                 " = __arguments[{i}].downcast_{maybe_opt}{root_identifier}();"
    //             )),
    //             _ => unreachable!(),
    //         },
    //         EntityRoutePtr::Custom(_) => {
    //             let parameter_ty_decl: Arc<TyDecl> =
    //                 self.db.ty_decl(parameter_ty.intrinsic()).unwrap();
    //             match parameter_ty_decl.ty_kind {
    //                 TyKind::Enum => self.write(&format!(
    //                     " = __arguments[{i}].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();"
    //                 )),
    //                 TyKind::Record => todo!(),
    //                 TyKind::Struct => todo!(),
    //                 TyKind::Primitive => todo!(),
    //                 TyKind::Vec => todo!(),
    //                 TyKind::Array => todo!(),
    //                 TyKind::Slice => todo!(),
    //                 TyKind::CyclicSlice => todo!(),
    //                 TyKind::Tuple => todo!(),
    //                 TyKind::Mor => todo!(),
    //                 TyKind::ThickFp =>{
    //                     self.write(&format!(
    //                     r#" = unsafe {{ __arguments[{i}]
    //                     .downcast_temp_ref::<__VirtualFunction>(&__registration__::__VIRTUAL_FUNCTION_VTABLE)
    //                     .downcast_thick_fp() }};"#
    //                     ));
    //                 },
    //                 TyKind::AssociatedAny => todo!(),
    //                 TyKind::ThisAny => todo!(),
    //                 TyKind::SpatialPlaceholderAny => todo!(),
    //                 TyKind::BoxAny => todo!(),
    //                 TyKind::HigherKind => todo!(),
    //                 TyKind::Ref => todo!(),
    //                 TyKind::Option => todo!(),
    //                 TyKind::TargetOutputAny => todo!(),
    //             }
    //         }
    //     }
    // }

    // fn gen_parameter_downcast_move(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_name = parameter.ident;
    //     let parameter_ty = parameter.ty();
    //     self.write(&format!(
    //         r#"
    //                 let {parameter_name}: "#
    //     ));
    //     self.gen_item_route(parameter_ty, EntityRouteRole::Decl);
    //     let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
    //     self.write(&format!(
    //         " = unsafe {{ __arb_ref(&__arguments[{i}]) }}.downcast_move(&__registration__::{mangled_parameter_ty_vtable});"
    //     ))
    // }

    // fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_name = parameter.ident;
    //     let parameter_ty = parameter.ty();
    //     self.write(&format!(
    //         r#"
    //                 let {parameter_name}: &"#
    //     ));
    //     self.gen_item_route(parameter_ty, EntityRouteRole::Decl);
    //     let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
    //     self.write(&format!(
    //         " = __arguments[{i}].downcast_temp_ref(&__registration__::{mangled_parameter_ty_vtable});"
    //     ))
    // }

    // fn gen_parameter_downcast_leash(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_name = parameter.ident;
    //     let parameter_ty = parameter.ty();
    //     self.write(&format!(
    //         r#"
    //                 let {parameter_name}: &'static "#
    //     ));
    //     let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
    //     self.gen_item_route(parameter_ty.intrinsic(), EntityRouteRole::Decl);
    //     self.write(&format!(" = __arguments[{i}].downcast_leash(&__registration__::{mangled_parameter_ty_vtable});"))
    // }
    // fn gen_parameter_downcast_opt_leash(&mut self, i: usize, parameter: &ParameterDecl) {
    //     let parameter_name = parameter.ident;
    //     let parameter_ty = parameter.ty();
    //     self.write(&format!(
    //         r#"
    //                 let {parameter_name}: Option<&'static "#
    //     ));
    //     let mangled_parameter_ty_vtable = self.db.mangled_intrinsic_ty_vtable(parameter_ty);
    //     self.gen_item_route(parameter_ty.intrinsic(), EntityRouteRole::Decl);
    //     self.write(&format!("> = __arguments[{i}].downcast_opt_leash(&__registration__::{mangled_parameter_ty_vtable});"))
    // }
}
