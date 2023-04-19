mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FormDefn {
    Fn(FnDefn),
    Val(ValDefn),
    Gn(GnDefn),
}

impl FormDefn {
    pub fn decl(self, db: &dyn DefnDb) -> FugitiveDecl {
        match self {
            FormDefn::Fn(defn) => defn.decl(db).into(),
            FormDefn::Val(defn) => defn.decl(db).into(),
            FormDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FugitivePath {
        match self {
            FormDefn::Fn(defn) => defn.path(db),
            FormDefn::Val(defn) => defn.path(db),
            FormDefn::Gn(defn) => defn.path(db),
        }
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FormDefn::Fn(defn) => defn.expr_region(db),
            FormDefn::Val(defn) => defn.expr_region(db),
            FormDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FugitiveDecl {
    type Defn = FormDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            FugitiveDecl::Fn(decl) => fn_defn(db, decl).into(),
            FugitiveDecl::Val(decl) => val_defn(db, decl).into(),
            FugitiveDecl::Gn(_) => todo!(),
        }
    }
}
