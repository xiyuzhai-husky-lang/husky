pub mod assoc_ritchie;
pub mod method_ritchie;

use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_eth_signature::signature::{assoc_item::trai_item::TraitItemEthTemplate, HasEthTemplate};

#[derive(Debug, PartialEq, Eq)]
pub enum TraitItemFlySignature {
    AssocType {
        path: TraitItemPath,
        trai: FlyTerm,
        item_ty: FlyTerm,
        instantiation: FlyInstantiation,
    },
}

/// # constructor

impl TraitItemFlySignature {
    pub(crate) fn from_path(
        path: TraitItemPath,
        self_ty: FlyTerm,
        engine: &mut impl FlyTermEngineMut,
    ) -> FlyTermResult<Self> {
        let db = engine.db();
        let eth_template = path.eth_template(db)?;
        let trai_path = path.trai_path(db);
        let trai_eth_template = trai_path.eth_template(db)?;
        match eth_template {
            TraitItemEthTemplate::AssocRitchie(_) => todo!(),
            TraitItemEthTemplate::AssocVal(_) => todo!(),
            TraitItemEthTemplate::AssocType(eth_template) => {
                p!(trai_path.debug(db));
                let trai = match trai_eth_template.template_parameters(db).len() {
                    0 => EthTerm::ItemPath(trai_path.into()).into(),
                    1 => todo!(),
                    _ => todo!(),
                };
                Ok(TraitItemFlySignature::AssocType {
                    path,
                    trai,
                    item_ty: engine.term_menu().ty0().into(),
                    instantiation: todo!(),
                })
            }
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
            TraitItemEthTemplate::MethodRitchie(_) => todo!(),
            TraitItemEthTemplate::MethodCurry(_) => todo!(),
        }
    }
}

/// # getters

impl TraitItemFlySignature {
    pub fn item_ty(&self) -> FlyTerm {
        match *self {
            TraitItemFlySignature::AssocType { item_ty, .. } => item_ty,
        }
    }
}
