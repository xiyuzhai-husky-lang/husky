use super::*;

impl<'a> RustCodeGenerator<'a> {
    // pub(super) fn gen_transfer_linkage(
    //     &mut self,
    //     needs_eval_context: bool,
    //     opt_this: Option<(ParameterModifier, EntityRoutePtr)>,
    //     gen_caller: impl FnOnce(&mut Self),
    //     gen_call_route: impl FnOnce(&mut Self),
    //     decl: &CallFugitiveDecl,
    // ) {
    //     let argidx_base = opt_this.map(|_| 1).unwrap_or(0);
    //     self.write(&format!(
    //         r#"
    //     transfer_linkage!(
    //         {{
    //             unsafe fn __wrapper<'eval>(
    //                 __arguments: &mut [__Register<'eval>],
    //                 __opt_ctx: Option<&dyn __EvalContext<'eval>>,
    //             ) -> __Register<'eval> {{"#
    //     ));
    //     if let Some((this_liason, this_ty)) = opt_this {
    //         let mangled_this_ty_vtable = self.db.mangled_intrinsic_ty_vtable(this_ty);
    //         match this_liason {
    //             ParameterModifier::None => {
    //                 self.write(&format!(
    //                     r#"
    //                 let __this: "#
    //                 ));
    //                 if self.db.is_copyable(this_ty).unwrap() {
    //                     todo!()
    //                 } else {
    //                     self.write("&");
    //                     self.gen_item_route(this_ty, EntityRouteRole::Decl);
    //                     self.write(&format!(" = __arguments[0].downcast_temp_ref(&__registration__::{mangled_this_ty_vtable});"))
    //                 }
    //             }
    //             ParameterModifier::Owned => todo!(),
    //             ParameterModifier::OwnedMut => todo!(),
    //             ParameterModifier::MemberAccess => panic!(),
    //             ParameterModifier::Leash => {
    //                 self.write(&format!(
    //                     r#"
    //                 let __this: "#
    //                 ));
    //                 if self.db.is_copyable(this_ty).unwrap() {
    //                     todo!()
    //                 } else {
    //                     self.write("&'eval ");
    //                     self.gen_item_route(this_ty.deref_route(), EntityRouteRole::Decl);
    //                     self.write(&format!(" = __arguments[0].downcast_eval_ref(&__registration__::{mangled_this_ty_vtable});"))
    //                 }
    //             }
    //             ParameterModifier::TempRef => todo!(),
    //             ParameterModifier::TempRefMut => {
    //                 self.write(&format!(
    //                     r#"
    //                 let __this: "#
    //                 ));
    //                 self.write("&mut ");
    //                 self.gen_item_route(this_ty, EntityRouteRole::Decl);
    //                 self.write(&format!(
    //                     " = unsafe {{ __arb_ref(&__arguments[0]) }}.downcast_temp_mut(&__registration__::{mangled_this_ty_vtable});"
    //                 ))
    //             }
    //         }
    //     }
    //     for (i, parameter) in decl.primary_parameters.iter().enumerate() {
    //         self.gen_parameter_downcast(i + argidx_base, parameter)
    //     }
    //     msg_once!("keyword parameter overrides");
    //     for (_i, parameter) in decl.keyword_parameters.iter().enumerate() {
    //         let parameter_name = parameter.ident;
    //         let parameter_ty = parameter.ty();
    //         self.write(&format!(
    //             r#"
    //                 let {parameter_name}: "#
    //         ));
    //         self.gen_item_route(parameter_ty, EntityRouteRole::Decl);
    //         self.write(&format!(" = todo!();"))
    //     }
    //     self.gen_variadics_downcast(decl);
    //     self.write(&format!(
    //         r#"
    //                 "#
    //     ));
    //     let return_ty = decl.output.ty();
    //     let canonical_return_ty = return_ty.canonicalize();
    //     let return_ty_reg_memory_kind = self.db.reg_memory_kind(return_ty);
    //     let is_intrinsic_return_ty_primitive = canonical_return_ty.is_intrinsic_route_primitive();
    //     match canonical_return_ty.kind() {
    //         CanonicalTyKind::Intrinsic => match return_ty_reg_memory_kind {
    //             RegMemoryKind::Direct => {
    //                 if is_intrinsic_return_ty_primitive {
    //                     // pass
    //                     ()
    //                 } else {
    //                     todo!()
    //                 }
    //             }
    //             RegMemoryKind::BoxCopyable | RegMemoryKind::BoxNonCopyable => {
    //                 self.write("__Register::new_box::<");
    //                 self.gen_item_route(
    //                     canonical_return_ty.intrinsic_ty(),
    //                     EntityRouteRole::Decl,
    //                 );
    //                 self.write(">(");
    //             }
    //         },
    //         CanonicalTyKind::Optional => match return_ty_reg_memory_kind {
    //             RegMemoryKind::Direct => {
    //                 if is_intrinsic_return_ty_primitive {
    //                     // pass
    //                     ()
    //                 } else {
    //                     todo!()
    //                 }
    //             }
    //             RegMemoryKind::BoxCopyable => todo!(),
    //             RegMemoryKind::BoxNonCopyable => todo!(),
    //         },
    //         CanonicalTyKind::Leash => todo!(),
    //         CanonicalTyKind::OptionalLeash => {
    //             self.write("__Register::new_opt_eval_ref::<");
    //             self.gen_item_route(canonical_return_ty.intrinsic_ty(), EntityRouteRole::Decl);
    //             self.write(">(");
    //         }
    //         CanonicalTyKind::TempRefMut => todo!(),
    //     }
    //     gen_caller(self);
    //     self.write("(");
    //     for (i, parameter) in decl.primary_parameters.iter().enumerate() {
    //         if i > 0 {
    //             self.write(", ")
    //         }
    //         self.write(&parameter.ident)
    //     }
    //     for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
    //         if i + decl.primary_parameters.len() > 0 {
    //             self.write(", ");
    //         }
    //         self.write(&parameter.ident)
    //     }
    //     match decl.variadic_parameters {
    //         VariadicParametersDecl::None => (),
    //         VariadicParametersDecl::RepeatSingle { .. } => {
    //             if decl.primary_parameters.len() > 0 || decl.keyword_parameters.len() > 0 {
    //                 self.write(", ")
    //             }
    //             self.write("__variadics")
    //         }
    //     }
    //     if needs_eval_context {
    //         if decl.primary_parameters.len() > 0
    //             || decl.keyword_parameters.len() > 0
    //             || decl.variadic_parameters.is_some()
    //         {
    //             self.write(", ")
    //         }
    //         self.write("__opt_ctx.unwrap()")
    //     }
    //     let mangled_return_ty_vtable = self.db.mangled_intrinsic_ty_vtable(decl.output.ty());
    //     if is_intrinsic_return_ty_primitive {
    //         self.write(&format!(
    //             r#").to_register()
    //             }}
    //             __wrapper
    //         }},
    //         some "#
    //         ));
    //     } else {
    //         self.write(&format!(
    //             r#"), &__registration__::{mangled_return_ty_vtable})
    //             }}
    //             __wrapper
    //         }},
    //         some "#
    //         ));
    //     }
    //     if needs_eval_context {
    //         self.write("ctx ")
    //     } else {
    //         self.write("base ")
    //     }
    //     gen_call_route(self);
    //     self.write(r#" as "#);
    //     self.gen_call_ty(needs_eval_context, decl);
    //     self.write(
    //         r#"
    //     ),"#,
    //     )
    // }

    // fn gen_variadics_downcast(&mut self, decl: &CallFugitiveDecl) {
    //     match decl.variadic_parameters {
    //         VariadicParametersDecl::None => (),
    //         VariadicParametersDecl::RepeatSingle {
    //             parameter: ref parameter_decl,
    //         } => {
    //             let variadic_start = decl.variadic_start();
    //             let variadic_ty = parameter_decl.ty();
    //             if variadic_ty.is_primitive() {
    //                 self.write(&format!(
    //                     r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v|v.downcast_{variadic_ty}())
    //                         .collect();"#,
    //                 ));
    //             } else {
    //                 let variadic_ty_vtable = self.db.mangled_intrinsic_ty_vtable(variadic_ty);
    //                 match self.db.is_copyable(variadic_ty).unwrap() {
    //                     true => {
    //                         if variadic_ty.is_option() {
    //                             let variadic_ty = variadic_ty.item_route_argument(0);
    //                             if variadic_ty.is_eval_ref() {
    //                                 self.write(&format!(
    //                                 r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v|v.downcast_opt_eval_ref(&__registration__::{variadic_ty_vtable}))
    //                         .collect();"#,
    //                 ));
    //                             } else if variadic_ty.is_primitive() {
    //                                 self.write(&format!(
    //                                     r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v|v.downcast_opt_{}())
    //                         .collect();"#,
    //                                     variadic_ty.ident().as_str()
    //                                 ));
    //                             } else {
    //                                 todo!()
    //                             }
    //                         } else if variadic_ty.is_eval_ref() {
    //                             self.write(&format!(
    //                                 r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v|v.downcast_eval_ref(&__registration__::{variadic_ty_vtable}))
    //                         .collect();"#,
    //                             ));
    //                         } else if variadic_ty.is_fp() {
    //                             self.write(&format!(
    //                                 r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v| {{
    //                             v.downcast_temp_ref::<__VirtualFunction>(
    //                                 &__registration__::{variadic_ty_vtable}
    //                             ).downcast_thick_fp()
    //                         }})
    //                         .collect();"#,
    //                             ));
    //                         } else {
    //                             p!(variadic_ty);
    //                             todo!()
    //                         }
    //                     }
    //                     false => {
    //                         if variadic_ty.is_option() {
    //                             todo!()
    //                         } else {
    //                             self.write(&format!(
    //                                 r#"
    //                 let __variadics =
    //                     __arguments[{variadic_start}..]
    //                         .iter_mut()
    //                         .map(|v|v.downcast_move(&__registration__::{variadic_ty_vtable}))
    //                         .collect();"#,
    //                             ));
    //                         }
    //                     }
    //                 };
    //             }
    //         }
    //     }
    // }

    // fn gen_call_ty(&mut self, needs_eval_context: bool, decl: &CallFugitiveDecl) {
    //     self.write("fn(");
    //     if let Some(this_ty) = decl.opt_this_ty() {
    //         match decl.opt_this_liason.unwrap() {
    //             ParameterModifier::None => {
    //                 if self.db.is_copyable(this_ty).unwrap() {
    //                     ()
    //                 } else {
    //                     self.write("&'static ")
    //                 }
    //             }
    //             ParameterModifier::Owned => todo!(),
    //             ParameterModifier::OwnedMut => todo!(),
    //             ParameterModifier::MemberAccess => todo!(),
    //             ParameterModifier::Leash => self.write("&'static "),
    //             ParameterModifier::TempRef => todo!(),
    //             ParameterModifier::TempRefMut => self.write("&'static mut "),
    //         }
    //         self.gen_item_route(this_ty, EntityRouteRole::StaticDecl)
    //     }
    //     for (i, parameter) in decl.primary_parameters.iter().enumerate() {
    //         if decl.opt_this_liason.is_some() || i > 0 {
    //             self.write(", ")
    //         }
    //         match parameter.modifier {
    //             ParameterModifier::None => {
    //                 if self.db.is_copyable(parameter.ty()).unwrap() {
    //                     ()
    //                 } else {
    //                     assert!(!parameter.ty().is_eval_ref());
    //                     self.write("&'static ")
    //                 }
    //             }
    //             ParameterModifier::Owned => (),
    //             ParameterModifier::OwnedMut => todo!(),
    //             ParameterModifier::MemberAccess => todo!(),
    //             ParameterModifier::Leash => {
    //                 assert!(!parameter.ty().is_eval_ref());
    //                 self.write("&'static ")
    //             }
    //             ParameterModifier::TempRef => todo!(),
    //             ParameterModifier::TempRefMut => todo!(),
    //         }
    //         self.gen_item_route(parameter.ty(), EntityRouteRole::StaticDecl)
    //     }
    //     for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
    //         if decl.opt_this_liason.is_some() || i + decl.primary_parameters.len() > 0 {
    //             self.write(", ");
    //         }
    //         match parameter.modifier {
    //             ParameterModifier::None => {
    //                 if self.db.is_copyable(parameter.ty()).unwrap() {
    //                     ()
    //                 } else {
    //                     self.write("&'static ")
    //                 }
    //             }
    //             ParameterModifier::Owned => (),
    //             ParameterModifier::OwnedMut => todo!(),
    //             ParameterModifier::MemberAccess => todo!(),
    //             ParameterModifier::Leash => self.write("&'static "),
    //             ParameterModifier::TempRef => todo!(),
    //             ParameterModifier::TempRefMut => todo!(),
    //         }
    //         self.gen_item_route(parameter.ty(), EntityRouteRole::StaticDecl)
    //     }
    //     match decl.variadic_parameters {
    //         VariadicParametersDecl::None => (),
    //         VariadicParametersDecl::RepeatSingle {
    //             parameter: ref parameter_decl,
    //         } => {
    //             if decl.opt_this_liason.is_some()
    //                 || decl.primary_parameters.len() > 0
    //                 || decl.keyword_parameters.len() > 0
    //             {
    //                 self.write(", ")
    //             }
    //             self.write("Vec<");
    //             self.gen_item_route(parameter_decl.ty(), EntityRouteRole::StaticDecl);
    //             self.write(">")
    //         }
    //     }
    //     if needs_eval_context {
    //         if decl.opt_this_liason.is_some()
    //             || decl.primary_parameters.len() > 0
    //             || decl.keyword_parameters.len() > 0
    //             || decl.variadic_parameters.is_some()
    //         {
    //             self.write(", ")
    //         }
    //         self.write("&dyn __EvalContext<'static>")
    //     }
    //     self.write(") -> ");
    //     self.gen_item_route(decl.output.ty(), EntityRouteRole::StaticDecl)
    // }
}
