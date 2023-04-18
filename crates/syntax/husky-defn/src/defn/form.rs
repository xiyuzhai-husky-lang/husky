mod feature;
mod function;
mod morphism;
mod type_alias;
mod value;

pub use feature::*;
pub use function::*;
pub use morphism::*;
pub use type_alias::*;
pub use value::*;

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
    pub fn decl(self, db: &dyn DefnDb) -> FormDecl {
        match self {
            FormDefn::Fn(defn) => defn.decl(db).into(),
            FormDefn::Val(defn) => defn.decl(db).into(),
            FormDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FormPath {
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

impl HasDefn for FormDecl {
    type Defn = FormDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            FormDecl::Fn(decl) => function_defn(db, decl).into(),
            FormDecl::Val(decl) => feature_defn(db, decl).into(),
            FormDecl::Gn(_) => todo!(),
        }
    }
}
