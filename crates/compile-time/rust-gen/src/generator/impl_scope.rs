use scope::*;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_scope(&mut self, scope: ScopePtr) {
        match scope.route {
            ScopeRoute::Builtin { ident } => self.result += &ident,
            ScopeRoute::Package { .. } => self.write("crate"),
            ScopeRoute::ChildScope { parent, ident } => {
                self.gen_scope(parent);
                self.write("::");
                self.write(&ident)
            }
            ScopeRoute::Implicit { main, ident } => todo!(),
        }
        if scope.generics.len() > 0 {
            todo!()
        }
    }
}
