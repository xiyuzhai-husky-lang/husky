use crate::*;

impl std::fmt::Display for EntityPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.opt_parent.is_none() {
            self.ident.fmt(f)
        } else {
            todo!()
        }
    }
}
