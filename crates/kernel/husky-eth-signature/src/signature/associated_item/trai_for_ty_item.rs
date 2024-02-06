mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEthTemplate {
    AssocFn(TraitForTypeAssocFnEthTemplate),
    AssocVal(TraitForTypeAssocValEthTemplate),
    AssocType(TraitForTypeAssocTypeEthTemplate),
    MethodFn(TraitForTypeMethodFnEthTemplate),
}

impl TraitForTypeItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitForTypeItemEthTemplate::AssocFn(_) => None,
            TraitForTypeItemEthTemplate::AssocVal(_) => None,
            TraitForTypeItemEthTemplate::AssocType(_) => None,
            TraitForTypeItemEthTemplate::MethodFn(template) => {
                // ad hoc
                Some(template.self_ty(db))
            }
        }
    }

    pub(crate) fn inherit_instantiation_builder(
        self,
        db: &::salsa::Db,
        impl_block_signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
    ) -> TraitForTypeItemEtherealSignatureBuilder {
        match self {
            TraitForTypeItemEthTemplate::AssocType(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::MethodFn(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::AssocFn(_) => todo!(),
            TraitForTypeItemEthTemplate::AssocVal(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureBuilder {
    AssocType(TraitForTypeAssocTypeEtherealSignatureBuilder),
    Method(TraitForTypeMethodFnEtherealSignatureBuilder),
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
        TraitForTypeItemDecTemplate::AssocFn(_) => todo!(),
        TraitForTypeItemDecTemplate::MethodFn(dec_template) => {
            TraitForTypeMethodFnEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocType(dec_template) => {
            TraitForTypeAssocTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocVal(_) => todo!(),
    })
}
