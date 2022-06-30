use entity_route::*;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, entity_route: EntityRoutePtr) {
        match entity_route.kind {
            EntityRouteKind::Root { ident } => self.result += &ident,
            EntityRouteKind::Package { .. } => self.write("crate"),
            EntityRouteKind::Child { parent, ident } => {
                self.gen_entity_route(parent);
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
        if entity_route.spatial_arguments.len() > 0 {
            self.write("<");
            for i in 0..entity_route.spatial_arguments.len() {
                if i > 0 {
                    self.write(", ")
                }
                match entity_route.spatial_arguments[i] {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(entity_route) => {
                        self.gen_entity_route(entity_route)
                    }
                }
            }
        }
    }
}
