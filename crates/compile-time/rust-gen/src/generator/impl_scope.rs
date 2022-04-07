use entity_route::*;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_scope(&mut self, scope: EntityRoutePtr) {
        match scope.kind {
            EntityRouteKind::Root { ident } => self.result += &ident,
            EntityRouteKind::Package { .. } => self.write("crate"),
            EntityRouteKind::ChildScope { parent, ident } => {
                self.gen_scope(parent);
                self.write("::");
                self.write(&ident)
            }
            EntityRouteKind::Contextual { main, ident } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
        }
        if scope.generics.len() > 0 {
            todo!()
        }
    }
}
