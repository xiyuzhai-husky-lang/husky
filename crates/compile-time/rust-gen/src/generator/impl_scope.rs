use entity_route::*;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_scope(&mut self, scope: EntityRoutePtr) {
        match scope.kind {
            EntityRouteKind::Root { ident } => self.result += &ident,
            EntityRouteKind::Package { .. } => self.write("crate"),
            EntityRouteKind::Child { parent, ident } => {
                self.gen_scope(parent);
                self.write("::");
                self.write(&ident)
            }
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        }
        if scope.generic_arguments.len() > 0 {
            todo!()
        }
    }
}
