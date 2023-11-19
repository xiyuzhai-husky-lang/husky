use super::*;
use husky_vfs::SubmodulePath;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn use_all_in_submodule(&mut self, submodule_path: SubmodulePath) {
        let db = self.db;
        self.on_fresh_semicolon_line(|builder| {
            builder.write_str("use self::");
            builder.write_str(submodule_path.ident(db).data(db));
            builder.write_str("::*")
        })
    }
}

impl<'a, 'b, HirEagerExprRegion> RustTranspilationBuilder<'a, 'b, HirEagerExprRegion> {
    pub(crate) fn one(&mut self) {
        self.write_str("1")
    }

    pub(crate) fn call_recv(&mut self) {
        self.write_str(".recv()")
    }

    pub(crate) fn use_all_in_crate(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use crate::*"))
    }

    pub(crate) fn use_all_in_super(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use super::*"))
    }
}
