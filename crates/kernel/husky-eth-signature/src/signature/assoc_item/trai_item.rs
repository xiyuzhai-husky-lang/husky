pub mod assoc_ritchie;
pub mod assoc_static_mut;
pub mod assoc_static_var;
pub mod assoc_ty;
pub mod assoc_val;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssocRitchie(TraitAssocRitchieEthTemplate),
    AssocVal(()),
    AssocType(()),
    AssocStaticMut(()),
    AssocStaticVar(()),
    MethodRitchie(()),
}

impl TraitItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemEthTemplate::AssocRitchie(slf) => slf.path(db),
            TraitItemEthTemplate::AssocType(_) => todo!(),
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
        }
    }

    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitItemEthTemplate::AssocRitchie(_) => None,
            TraitItemEthTemplate::AssocType(_) => todo!(),
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn eth_template(self, _db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        todo!()
    }
}
