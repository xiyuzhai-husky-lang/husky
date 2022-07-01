use entity_route::*;
use word::RootIdentifier;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityRouteRole {
    Caller,
    SpatialArgument,
    Value,
    Decl,
}

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, entity_route: EntityRoutePtr, role: EntityRouteRole) {
        match entity_route.kind {
            EntityRouteKind::Root { ident } => match ident {
                RootIdentifier::Void => self.write("()"),
                RootIdentifier::B32 => self.write("u32"),
                RootIdentifier::B64 => self.write("u64"),
                RootIdentifier::Std => self.write("__std"),
                RootIdentifier::Ref => {
                    self.write("&'eval ");
                    self.gen_entity_route(
                        entity_route.spatial_arguments[0].take_entity_route(),
                        EntityRouteRole::SpatialArgument,
                    );
                    return;
                }
                _ => self.result += &ident,
            },
            EntityRouteKind::Package { .. } => self.write("crate"),
            EntityRouteKind::Child { parent, ident } => {
                self.gen_entity_route(parent, role);
                self.write("::");
                self.write(&ident)
            }
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TypeAsTraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        }
        let contains_eval_ref = self.db.contains_eval_ref(entity_route.kind);
        if contains_eval_ref || entity_route.spatial_arguments.len() > 0 {
            match role {
                EntityRouteRole::Caller => self.write("::"),
                EntityRouteRole::SpatialArgument => (),
                EntityRouteRole::Value => (),
                EntityRouteRole::Decl => (),
            }
            self.write("<");
            if contains_eval_ref {
                self.write("'eval");
            }
            for i in 0..entity_route.spatial_arguments.len() {
                if i > 0 || contains_eval_ref {
                    self.write(", ")
                }
                match entity_route.spatial_arguments[i] {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(entity_route) => {
                        self.gen_entity_route(entity_route, EntityRouteRole::SpatialArgument)
                    }
                }
            }
            self.write(">");
        }
    }
}
