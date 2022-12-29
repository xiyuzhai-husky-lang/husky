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
}

impl From<ValueDefn> for FormDefn {
    fn from(v: ValueDefn) -> Self {
        Self::Value(v)
    }
}

impl From<MorphismDefn> for FormDefn {
    fn from(v: MorphismDefn) -> Self {
        Self::Morphism(v)
    }
}

impl From<FeatureDefn> for FormDefn {
    fn from(v: FeatureDefn) -> Self {
        Self::Feature(v)
    }
}

impl From<FunctionDefn> for FormDefn {
    fn from(v: FunctionDefn) -> Self {
        Self::Function(v)
    }
}
