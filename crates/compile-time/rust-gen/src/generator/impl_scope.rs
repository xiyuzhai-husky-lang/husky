use entity_route::*;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_scope(&mut self, scope: EntityRoutePtr) {
        match scope.kind {
            ScopeKind::Builtin { ident } => self.result += &ident,
            ScopeKind::Package { .. } => self.write("crate"),
            ScopeKind::ChildScope { parent, ident } => {
                self.gen_scope(parent);
                self.write("::");
                self.write(&ident)
            }
            ScopeKind::Contextual { main, ident } => todo!(),
            ScopeKind::Generic { ident, .. } => todo!(),
        }
        if scope.generics.len() > 0 {
            todo!()
        }
    }
}
