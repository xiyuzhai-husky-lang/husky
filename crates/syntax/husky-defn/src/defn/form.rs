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

impl FormDefn {
    pub fn module_item_path(self, db: &dyn DefnDb) -> ModuleItemPath {
        match self {
            FormDefn::Function(defn) => defn.module_item_path(db),
            FormDefn::Feature(defn) => defn.module_item_path(db),
            FormDefn::Morphism(defn) => defn.module_item_path(db),
            FormDefn::Value(defn) => defn.module_item_path(db),
        }
    }
}
