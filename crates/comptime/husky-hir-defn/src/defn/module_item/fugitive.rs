mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveHirDefn {
    Fn(FnHirDefn),
    // Function(FunctionDefn),
    Val(ValHirDefn),
    Gn(GnHirDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveHirDefn {
    pub fn decl(self, db: &dyn HirDefnDb) -> FugitiveHirDecl {
        match self {
            FugitiveHirDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveHirDefn::Val(defn) => defn.decl(db).into(),
            FugitiveHirDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn HirDefnDb) -> FugitivePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }
    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> HirExprRegion {
        match self {
            FugitiveHirDefn::Fn(defn) => defn.expr_region(db),
            FugitiveHirDefn::Val(defn) => defn.expr_region(db),
            FugitiveHirDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasHirDefn for FugitivePath {
    type HirDefn = FugitiveHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        fugitive_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn fugitive_hir_defn(db: &dyn HirDefnDb, path: FugitivePath) -> FugitiveHirDefn {
    match path.hir_decl(db) {
        FugitiveHirDecl::Fn(decl) => FnHirDefn::new(db, path, decl).into(),
        FugitiveHirDecl::Val(decl) => ValHirDefn::new(db, path, decl).into(),
        FugitiveHirDecl::Gn(decl) => GnHirDefn::new(db, path, decl).into(),
    }
}
