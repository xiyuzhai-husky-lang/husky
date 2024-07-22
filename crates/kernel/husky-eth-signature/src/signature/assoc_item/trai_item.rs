pub mod assoc_ritchie;
pub mod assoc_static_mut;
pub mod assoc_static_var;
pub mod assoc_ty;
pub mod assoc_val;
pub mod memo;
pub mod method_curry;
pub mod method_ritchie;

use self::{
    assoc_ritchie::TraitAssocRitchieEthTemplate, assoc_static_mut::TraitAssocStaticMutEthTemplate,
    assoc_static_var::TraitAssocStaticVarEthTemplate, assoc_ty::TraitAssocTypeEthTemplate,
    assoc_val::TraitAssocValEthTemplate, method_curry::TraitMethodCurryEthTemplate,
    method_ritchie::TraitMethodRitchieEthTemplate,
};
use super::*;
use husky_dec_signature::signature::assoc_item::trai_item::TraitItemDecTemplate;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TraitItemEthTemplate {
    AssocRitchie(TraitAssocRitchieEthTemplate),
    AssocVal(TraitAssocValEthTemplate),
    AssocType(TraitAssocTypeEthTemplate),
    AssocStaticMut(TraitAssocStaticMutEthTemplate),
    AssocStaticVar(TraitAssocStaticVarEthTemplate),
    MethodRitchie(TraitMethodRitchieEthTemplate),
    MethodCurry(TraitMethodCurryEthTemplate),
}

impl TraitItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemEthTemplate::AssocRitchie(slf) => slf.path(db),
            TraitItemEthTemplate::AssocType(_) => todo!(),
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
            TraitItemEthTemplate::AssocVal(_) => todo!(),
            TraitItemEthTemplate::MethodRitchie(_) => todo!(),
            TraitItemEthTemplate::MethodCurry(_) => todo!(),
        }
    }

    pub fn self_ty(self, _db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitItemEthTemplate::AssocRitchie(_) => None,
            TraitItemEthTemplate::AssocType(_) => todo!(),
            TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
            TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
            TraitItemEthTemplate::AssocVal(_) => todo!(),
            TraitItemEthTemplate::MethodRitchie(_) => todo!(),
            TraitItemEthTemplate::MethodCurry(_) => todo!(),
        }
    }
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        match self {
            TraitItemEthTemplate::AssocRitchie(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::AssocVal(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::AssocType(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::AssocStaticMut(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::AssocStaticVar(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::MethodRitchie(slf) => slf.template_parameters(db),
            TraitItemEthTemplate::MethodCurry(slf) => slf.template_parameters(db),
        }
    }
}

impl HasEthTemplate for TraitItemPath {
    type EthTemplate = TraitItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        trai_item_eth_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn trai_item_eth_template(
    db: &::salsa::Db,
    path: TraitItemPath,
) -> EthSignatureResult<TraitItemEthTemplate> {
    Ok(match path.dec_template(db)? {
        TraitItemDecTemplate::AssocRitchie(_) => todo!(),
        TraitItemDecTemplate::MethodRitchie(_) => todo!(),
        TraitItemDecTemplate::AssocType(template) => {
            TraitAssocTypeEthTemplate::from_dec(db, path, template)?.into()
        }
        TraitItemDecTemplate::AssocStaticMut(_) => todo!(),
        TraitItemDecTemplate::AssocStaticVar(template) => {
            TraitAssocStaticVarEthTemplate::from_dec(path, template, db)?.into()
        }
        TraitItemDecTemplate::AssocVal(_) => todo!(),
    })
}
