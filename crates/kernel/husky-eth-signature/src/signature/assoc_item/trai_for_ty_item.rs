mod assoc_ritchie;
mod assoc_ty;
mod assoc_val;
mod method_curry;
mod method_ritchie;

pub use self::assoc_ritchie::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_ritchie::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEthTemplate {
    AssocRitchie(TraitForTypeAssocRitchieEthTemplate),
    AssocVal(TraitForTypeAssocValEthTemplate),
    AssocType(TraitForTypeAssocTypeEthTemplate),
    MethodRitchie(TraitForTypeMethodRitchieEthTemplate),
}

impl TraitForTypeItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitForTypeItemEthTemplate::AssocRitchie(_) => None,
            TraitForTypeItemEthTemplate::AssocVal(_) => None,
            TraitForTypeItemEthTemplate::AssocType(_) => None,
            TraitForTypeItemEthTemplate::MethodRitchie(template) => {
                // ad hoc
                Some(template.self_ty(db))
            }
        }
    }

    pub(crate) fn inherit_instantiation_builder(
        self,
        db: &::salsa::Db,
        impl_block_signature_builder: EthTraitForTypeImplBlockSignatureBuilder,
    ) -> TraitForTypeItemEtherealSignatureBuilder {
        match self {
            TraitForTypeItemEthTemplate::AssocType(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::MethodRitchie(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::AssocRitchie(_) => todo!(),
            TraitForTypeItemEthTemplate::AssocVal(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureBuilder {
    AssocType(TraitForTypeAssocTypeEtherealSignatureBuilder),
    Method(TraitForTypeMethodRitchieEtherealSignatureBuilder),
}

impl HasEthTemplate for TraitForTypeItemPath {
    type EthTemplate = TraitForTypeItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        trai_for_ty_item_eth_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_item_eth_template(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> EtherealSignatureResult<TraitForTypeItemEthTemplate> {
    Ok(match path.dec_template(db)? {
        TraitForTypeItemDecTemplate::AssocRitchie(_) => todo!(),
        TraitForTypeItemDecTemplate::MethodRitchie(dec_template) => {
            TraitForTypeMethodRitchieEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocType(dec_template) => {
            TraitForTypeAssocTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocVal(_) => todo!(),
    })
}
