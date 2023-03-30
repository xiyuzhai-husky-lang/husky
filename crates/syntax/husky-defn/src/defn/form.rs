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
    Function(FunctionDefn),
    Feature(FeatureDefn),
    Morphism(MorphismDefn),
    Value(ValueDefn),
}

impl FormDefn {
    pub fn decl(self, db: &dyn DefnDb) -> FormDecl {
        match self {
            FormDefn::Function(defn) => defn.decl(db).into(),
            FormDefn::Feature(defn) => defn.decl(db).into(),
            FormDefn::Morphism(defn) => defn.decl(db).into(),
            FormDefn::Value(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FormPath {
        match self {
            FormDefn::Function(defn) => defn.path(db),
            FormDefn::Feature(defn) => defn.path(db),
            FormDefn::Morphism(defn) => defn.path(db),
            FormDefn::Value(defn) => defn.path(db),
        }
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FormDefn::Function(defn) => defn.expr_region(db),
            FormDefn::Feature(defn) => defn.expr_region(db),
            FormDefn::Morphism(defn) => defn.expr_region(db),
            FormDefn::Value(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FormDecl {
    type Defn = FormDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            FormDecl::Fn(decl) => function_defn(db, decl).into(),
            FormDecl::Feature(decl) => feature_defn(db, decl).into(),
            FormDecl::Gn(_) => todo!(),
            FormDecl::Value(_) => todo!(),
        }
    }
}
