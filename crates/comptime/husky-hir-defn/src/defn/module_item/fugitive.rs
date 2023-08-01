mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveHirDefn {
    Fn(FnHirDefn),
    // Function(FunctionDefn),
    Val(ValHirDefn),
    Gn(GnHirDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> FugitiveHirDecl {
        match self {
            FugitiveHirDefn::Fn(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.hir_decl(db).into(),
            FugitiveHirDefn::Gn(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn HirDefnDb) -> FugitivePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(hir_defn) => hir_defn.path(db),
        //     FugitiveDefn::Val(hir_defn) => hir_defn.path(db),
        //     FugitiveDefn::Gn(hir_defn) => hir_defn.path(db),
        // }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> HirExprRegion {
        match self {
            FugitiveHirDefn::Fn(hir_defn) => hir_defn.hir_expr_region(db).into(),
            FugitiveHirDefn::Val(hir_defn) => hir_defn.hir_expr_region(db).into(),
            FugitiveHirDefn::Gn(hir_defn) => hir_defn.hir_expr_region(db).into(),
        }
    }
}

impl HasHirDefn for FugitivePath {
    type HirDefn = FugitiveHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        fugitive_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn fugitive_hir_defn(db: &dyn HirDefnDb, path: FugitivePath) -> Option<FugitiveHirDefn> {
    match path.hir_decl(db)? {
        FugitiveHirDecl::Fn(hir_decl) => Some(FnHirDefn::new(db, path, hir_decl).into()),
        FugitiveHirDecl::Val(hir_decl) => Some(ValHirDefn::new(db, path, hir_decl).into()),
        FugitiveHirDecl::Gn(hir_decl) => Some(GnHirDefn::new(db, path, hir_decl).into()),
        FugitiveHirDecl::TypeAlias(_) => todo!(),
    }
}
