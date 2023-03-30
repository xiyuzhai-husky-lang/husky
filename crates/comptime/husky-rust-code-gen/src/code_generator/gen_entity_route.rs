use husky_term::*;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, _entity_route: Term, _role: EntityRouteRole) {
        todo!()
        // if let Some(_) = self
        //     .entity_route_uses
        //     .find(|candidate| candidate.variant == entity_path.variant)
        // {
        //     self.write(&entity_path.ident())
        // } else {
        //     match entity_path.variant {
        //         EntityRouteVariant::Root { ident } => match ident {
        //             RootBuiltinIdent::Void => self.write("()"),
        //             RootBuiltinIdent::B32 => self.write("u32"),
        //             RootBuiltinIdent::B64 => self.write("u64"),
        //             RootBuiltinIdent::Std => self.write("__std"),
        //             RootBuiltinIdent::ThickFp => {
        //                 match role {
        //                     EntityRouteRole::StaticThinFpTyDecl { .. } => self.write("fn("),
        //                     _ => self.write("ThickFp<fn("),
        //                 }
        //                 for i in 0..(entity_path.spatial_arguments.len() - 1) {
        //                     if i > 0 {
        //                         self.write(", ")
        //                     }
        //                     let argument_ty = entity_path.spatial_arguments[i].take_entity_route();
        //                     if !self.db.is_copyable(argument_ty).unwrap() {
        //                         self.write("&")
        //                     }
        //                     self.gen_entity_route(argument_ty, EntityRouteRole::StaticDecl)
        //                 }
        //                 if entity_path.ident().as_str() == "downmost" {
        //                     p!(self.db.needs_eval_context(entity_path));
        //                     todo!()
        //                 }
        //                 match role {
        //                     EntityRouteRole::StaticThinFpTyDecl {
        //                         needs_eval_context: true,
        //                     } => {
        //                         if entity_path.spatial_arguments.len() > 1 {
        //                             self.write(", ")
        //                         }
        //                         self.write("&dyn __EvalContext<'static>")
        //                     }
        //                     _ => (),
        //                 }
        //                 self.write(")");
        //                 let return_ty = entity_path
        //                     .spatial_arguments
        //                     .last()
        //                     .unwrap()
        //                     .take_entity_route();
        //                 if return_ty != EntityRoutePtr::Root(RootBuiltinIdent::Void) {
        //                     self.write("->");
        //                     self.gen_entity_route(return_ty, EntityRouteRole::StaticDecl)
        //                 }
        //                 match role {
        //                     EntityRouteRole::StaticThinFpTyDecl { .. } => self.write(""),
        //                     _ => self.write(">"),
        //                 }
        //                 return;
        //             }
        //             RootBuiltinIdent::FnMut => todo!(),
        //             RootBuiltinIdent::Fn => todo!(),
        //             RootBuiltinIdent::FnOnce => todo!(),
        //             RootBuiltinIdent::Ref => {
        //                 match role {
        //                     EntityRouteRole::StaticCallRoute
        //                     | EntityRouteRole::ForAnyLifetimeOther => self.write("&"),
        //                     EntityRouteRole::Caller
        //                     | EntityRouteRole::Decl
        //                     | EntityRouteRole::Other => self.write("&'eval "),
        //                     EntityRouteRole::StaticDecl => self.write("&'static "),
        //                     EntityRouteRole::StaticThinFpTyDecl { .. } => self.write("&'static "),
        //                     EntityRouteRole::FpValue => todo!(),
        //                 }
        //                 self.gen_entity_route(
        //                     entity_path.entity_route_argument(0),
        //                     role.argument_role(),
        //                 );
        //                 return;
        //             }
        //             _ => self.result += &ident,
        //         },
        //         EntityRouteVariant::Package { .. } => self.write("crate"),
        //         EntityRouteVariant::Child { parent, ident } => {
        //             self.gen_entity_route(parent, role.parent_role());
        //             self.write("::");
        //             self.write(&ident);
        //             // ad hoc
        //             if ident.as_str() == "pop_with_largest_opt_f32" {
        //                 let elem_ty = parent.entity_route_argument(0);
        //                 if self.db.is_copyable(elem_ty).unwrap() {
        //                     self.write("_copyable")
        //                 } else {
        //                     self.write("_borrow")
        //                 }
        //             }
        //         }
        //         EntityRouteVariant::TargetInputValue => self.write("__input"),
        //         EntityRouteVariant::Any { .. } => {
        //             p!(entity_path);
        //             todo!()
        //         }
        //         EntityRouteVariant::ThisType { .. } => todo!(),
        //         EntityRouteVariant::TraitForTypeMember { .. } => todo!(),
        //         EntityRouteVariant::TargetOutputType => todo!(),
        //     }
        // }
        // let needs_eval_ref = match role {
        //     EntityRouteRole::Decl
        //     | EntityRouteRole::StaticThinFpTyDecl { .. }
        //     | EntityRouteRole::StaticDecl => {
        //         self.db.entity_route_variant_contains_eval_ref(entity_path)
        //     }
        //     _ => false,
        // };
        // if needs_eval_ref || entity_path.spatial_arguments.len() > 0 {
        //     match role {
        //         EntityRouteRole::Caller | EntityRouteRole::StaticCallRoute => self.write("::"),
        //         _ => (),
        //     }
        //     self.write("<");
        //     if needs_eval_ref {
        //         match role {
        //             EntityRouteRole::Decl => self.write("'eval"),
        //             EntityRouteRole::StaticThinFpTyDecl { .. } | EntityRouteRole::StaticDecl => {
        //                 self.write("'static")
        //             }
        //             _ => panic!(),
        //         }
        //     }
        //     for i in 0..entity_path.spatial_arguments.len() {
        //         if i > 0 || needs_eval_ref {
        //             self.write(", ")
        //         }
        //         match entity_path.spatial_arguments[i] {
        //             SpatialArgument::Const(_) => todo!(),
        //             SpatialArgument::EntityRoute(entity_path) => {
        //                 self.gen_entity_route(entity_path, role.argument_role())
        //             }
        //         }
        //     }
        //     self.write(">");
        // }
    }
}
