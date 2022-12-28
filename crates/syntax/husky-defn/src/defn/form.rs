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
    pub fn entity_path(self, db: &dyn DefnDb) -> EntityPath {
        match self {
            FormDefn::Function(defn) => defn.entity_path(db),
            FormDefn::Feature(defn) => defn.entity_path(db),
            FormDefn::Morphism(defn) => defn.entity_path(db),
            FormDefn::Value(defn) => defn.entity_path(db),
        }
    }
}
