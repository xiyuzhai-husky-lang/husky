use husky_entity_route::*;
use word::RootIdentifier;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityRouteRole {
    Caller,
    Decl,
    Other,
}

impl EntityRouteRole {
    fn argument_role(self) -> Self {
        match self {
            EntityRouteRole::Caller => EntityRouteRole::Other,
            EntityRouteRole::Decl => EntityRouteRole::Decl,
            EntityRouteRole::Other => EntityRouteRole::Other,
        }
    }
}

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, entity_route: EntityRoutePtr, role: EntityRouteRole) {
        if let Some(_) = self
            .entity_route_uses
            .find(|candidate| candidate.kind == entity_route.kind)
        {
            self.write(&entity_route.ident())
        } else {
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
                            role.argument_role(),
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
                EntityRouteKind::Input { main } => self.write("ctx.input"),
                EntityRouteKind::Generic { ident, .. } => {
                    p!(entity_route);
                    todo!()
                }
                EntityRouteKind::ThisType => todo!(),
                EntityRouteKind::TypeAsTraitMember {
                    ty: parent,
                    trai,
                    ident,
                } => todo!(),
            }
        }
        let needs_eval_ref = match role {
            EntityRouteRole::Decl => self
                .db
                .entity_route_kind_contains_eval_ref(entity_route.kind),
            _ => false,
        };
        if needs_eval_ref || entity_route.spatial_arguments.len() > 0 {
            match role {
                EntityRouteRole::Caller => self.write("::"),
                _ => (),
            }
            self.write("<");
            if needs_eval_ref {
                self.write("'eval");
            }
            for i in 0..entity_route.spatial_arguments.len() {
                if i > 0 || needs_eval_ref {
                    self.write(", ")
                }
                match entity_route.spatial_arguments[i] {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(entity_route) => {
                        self.gen_entity_route(entity_route, role.argument_role())
                    }
                }
            }
            self.write(">");
        }
    }
}
