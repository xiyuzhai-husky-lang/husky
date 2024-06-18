pub mod assoc_ritchie;
pub mod method_ritchie;

use path::assoc_item::ty_item::TypeItemPath;

use self::assoc_ritchie::TypeAssocRitchieFlySignature;
use super::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeItemFlySignature {
    AssocRitchie(TypeAssocRitchieFlySignature),
}

impl TypeItemFlySignature {
    pub fn item_ty(&self) -> FlyTerm {
        match self {
            TypeItemFlySignature::AssocRitchie(slf) => slf.ty(),
        }
    }

    pub fn path(&self) -> TypeItemPath {
        match self {
            TypeItemFlySignature::AssocRitchie(slf) => slf.path(),
        }
    }

    pub fn instantiation(&self) -> &FlyInstantiation {
        match self {
            TypeItemFlySignature::AssocRitchie(slf) => slf.instantiation(),
        }
    }
}
