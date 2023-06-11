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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FugitiveDefn {
    Fn(FnDefn),
    // Function(FunctionDefn),
    Val(ValDefn),
    Gn(GnDefn),
    // TypeAlias(TypeAliasDefn)
}

impl FugitiveDefn {
    pub fn decl(self, db: &dyn DefnDb) -> FugitiveDecl {
        match self {
            FugitiveDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveDefn::Val(defn) => defn.decl(db).into(),
            FugitiveDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FugitivePath {
        match self {
            FugitiveDefn::Fn(defn) => defn.path(db),
            FugitiveDefn::Val(defn) => defn.path(db),
            FugitiveDefn::Gn(defn) => defn.path(db),
        }
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FugitiveDefn::Fn(defn) => defn.expr_region(db),
            FugitiveDefn::Val(defn) => defn.expr_region(db),
            FugitiveDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FugitiveDecl {
    type Defn = FugitiveDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            FugitiveDecl::Fn(decl) => decl.defn(db).into(),
            FugitiveDecl::Val(decl) => decl.defn(db).into(),
            FugitiveDecl::Gn(decl) => decl.defn(db).into(),
        }
    }
}
