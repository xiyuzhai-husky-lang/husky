use husky_ethereal_term::*;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_item_route(&mut self, _item_route: EtherealTerm, _role: EntityRouteRole) {
        todo!()
        // if let Some(_) = self
        //     .item_route_uses
        //     .find(|candidate| candidate.variant == item_path.variant)
        // {
        //     self.write(&item_path.ident())
        // } else {
        //     match item_path.variant {
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
        //                 for i in 0..(item_path.spatial_arguments.len() - 1) {
        //                     if i > 0 {
        //                         self.write(", ")
        //                     }
        //                     let argument_ty = item_path.spatial_arguments[i].take_item_route();
        //                     if !self.db.is_copyable(argument_ty).unwrap() {
        //                         self.write("&")
        //                     }
        //                     self.gen_item_route(argument_ty, EntityRouteRole::StaticDecl)
        //                 }
        //                 if item_path.ident().as_str() == "downmost" {
        //                     p!(self.db.needs_eval_context(item_path));
        //                     todo!()
        //                 }
        //                 match role {
        //                     EntityRouteRole::StaticThinFpTyDecl {
        //                         needs_eval_context: true,
        //                     } => {
        //                         if item_path.spatial_arguments.len() > 1 {
        //                             self.write(", ")
        //                         }
        //                         self.write("&dyn __EvalContext")
        //                     }
        //                     _ => (),
        //                 }
        //                 self.write(")");
        //                 let return_ty = item_path
        //                     .spatial_arguments
        //                     .last()
        //                     .unwrap()
        //                     .take_item_route();
        //                 if return_ty != EntityRoutePtr::Root(RootBuiltinIdent::Void) {
        //                     self.write("->");
        //                     self.gen_item_route(return_ty, EntityRouteRole::StaticDecl)
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
        //                     | EntityRouteRole::Other => self.write("&'static "),
        //                     EntityRouteRole::StaticDecl => self.write("&'static "),
        //                     EntityRouteRole::StaticThinFpTyDecl { .. } => self.write("&'static "),
        //                     EntityRouteRole::FpValue => todo!(),
        //                 }
        //                 self.gen_item_route(
        //                     item_path.item_route_argument(0),
        //                     role.argument_role(),
        //                 );
        //                 return;
        //             }
        //             _ => self.result += &ident,
        //         },
        //         EntityRouteVariant::Package { .. } => self.write("crate"),
        //         EntityRouteVariant::Child { parent, ident } => {
        //             self.gen_item_route(parent, role.parent_role());
        //             self.write("::");
        //             self.write(&ident);
        //             // ad hoc
        //             if ident.as_str() == "pop_with_largest_opt_f32" {
        //                 let elem_ty = parent.item_route_argument(0);
        //                 if self.db.is_copyable(elem_ty).unwrap() {
        //                     self.write("_copyable")
        //                 } else {
        //                     self.write("_borrow")
        //                 }
        //             }
        //         }
        //         EntityRouteVariant::TargetInputValue => self.write("__input"),
        //         EntityRouteVariant::Any { .. } => {
        //             p!(item_path);
        //             todo!()
        //         }
        //         EntityRouteVariant::ThisType { .. } => todo!(),
        //         EntityRouteVariant::TraitForTypeMember { .. } => todo!(),
        //         EntityRouteVariant::TargetOutputType => todo!(),
        //     }
        // }
        // let needs_leash = match role {
        //     EntityRouteRole::Decl
        //     | EntityRouteRole::StaticThinFpTyDecl { .. }
        //     | EntityRouteRole::StaticDecl => {
        //         self.db.item_route_variant_contains_leash(item_path)
        //     }
        //     _ => false,
        // };
        // if needs_leash || item_path.spatial_arguments.len() > 0 {
        //     match role {
        //         EntityRouteRole::Caller | EntityRouteRole::StaticCallRoute => self.write("::"),
        //         _ => (),
        //     }
        //     self.write("<");
        //     if needs_leash {
        //         match role {
        //             EntityRouteRole::Decl => self.write("'static"),
        //             EntityRouteRole::StaticThinFpTyDecl { .. } | EntityRouteRole::StaticDecl => {
        //                 self.write("'static")
        //             }
        //             _ => panic!(),
        //         }
        //     }
        //     for i in 0..item_path.spatial_arguments.len() {
        //         if i > 0 || needs_leash {
        //             self.write(", ")
        //         }
        //         match item_path.spatial_arguments[i] {
        //             SpatialArgument::Const(_) => todo!(),
        //             SpatialArgument::EntityRoute(item_path) => {
        //                 self.gen_item_route(item_path, role.argument_role())
        //             }
        //         }
        //     }
        //     self.write(">");
        // }
    }
}
