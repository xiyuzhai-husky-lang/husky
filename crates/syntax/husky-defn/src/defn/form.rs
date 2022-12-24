mod constant;
mod feature;
mod function;
mod method;
mod morphism;
mod type_alias;

pub use constant::*;
pub use feature::*;
pub use function::*;
pub use method::*;
pub use morphism::*;
pub use type_alias::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FormDefn {
    Function(FunctionDefn),
    Method(MethodDefn),
    Feature(FeatureDefn),
    Morphism(MorphismDefn),
    Const(ConstantDefn),
    TypeAlias(TypeAliasDefn),
}

impl FormDefn {
    pub fn entity_path(self, db: &dyn DefnDb) -> EntityPath {
        match self {
            FormDefn::Function(defn) => defn.entity_path(db),
            FormDefn::Method(defn) => defn.entity_path(db),
            FormDefn::Feature(defn) => defn.entity_path(db),
            FormDefn::Morphism(defn) => defn.entity_path(db),
            FormDefn::Const(defn) => defn.entity_path(db),
            FormDefn::TypeAlias(defn) => defn.entity_path(db),
        }
    }
}
