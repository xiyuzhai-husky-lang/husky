use super::*;
use husky_entity_path::SubmoduleItemPath;
use husky_hir_defn::{HasHirDefn, TypeHirDefn};
use husky_manifest::PackageDependency;
use husky_vfs::SubmodulePath;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn pub_use_all_in_submodule(&mut self, submodule_path: SubmoduleItemPath) {
        let db = self.db;
        self.on_fresh_semicolon_line(|builder| {
            builder.write_str("pub use self::");
            builder.write_str(submodule_path.ident(db).data(db));
            builder.write_str("::*")
        })
    }

    pub(crate) fn use_all_in_dep(&mut self, dep: &PackageDependency) {
        let db = self.db;
        self.on_fresh_semicolon_line(|builder| {
            builder.write_str("use ");
            builder.write_str(dep.package_path().ident(db).data(db));
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

    pub(crate) fn ty_constructor(&mut self, ty_path: TypePath) {
        ty_path.transpile_to_rust(self);
        match ty_path.hir_defn(self.db).unwrap() {
            TypeHirDefn::PropsStruct(_) => self.write_str("::__constructor"),
            TypeHirDefn::TupleStruct(_) => (),
            _ => unreachable!(),
        }
    }

    pub(crate) fn use_all_in_crate(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use crate::*"))
    }

    pub(crate) fn use_all_in_super(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use super::*"))
    }

    pub(crate) fn r8(&mut self) {
        self.write_str("r8")
    }

    pub(crate) fn r16(&mut self) {
        self.write_str("u16")
    }

    pub(crate) fn r32(&mut self) {
        self.write_str("u32")
    }

    pub(crate) fn r64(&mut self) {
        self.write_str("u64")
    }

    pub(crate) fn r128(&mut self) {
        self.write_str("u128")
    }

    pub(crate) fn rsize(&mut self) {
        self.write_str("usize")
    }

    pub(crate) fn usize(&mut self) {
        self.write_str("usize")
    }

    pub(crate) fn unit(&mut self) {
        self.write_str("()")
    }

    pub(crate) fn wrap_in_some_left(&mut self, wrap_in_some_flag: &mut bool) {
        debug_assert!(!*wrap_in_some_flag);
        *wrap_in_some_flag = true;
        self.write_str("Some(")
    }

    pub(crate) fn wrap_in_some_right(&mut self) {
        self.write_str(")")
    }
}
