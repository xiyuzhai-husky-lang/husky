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
    AssociatedFn(TraitForTypeAssociatedFnEthTemplate),
    AssociatedVal(TraitForTypeAssociatedValEthTemplate),
    AssociatedType(TraitForTypeAssociatedTypeEthTemplate),
    MethodFn(TraitForTypeMethodFnEthTemplate),
}

impl TraitForTypeItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            TraitForTypeItemEthTemplate::AssociatedFn(_) => None,
            TraitForTypeItemEthTemplate::AssociatedVal(_) => None,
            TraitForTypeItemEthTemplate::AssociatedType(_) => None,
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
            TraitForTypeItemEthTemplate::AssociatedType(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::MethodFn(item_template) => item_template
                .inherit_instantiation_builder(db, impl_block_signature_builder)
                .into(),
            TraitForTypeItemEthTemplate::AssociatedFn(_) => todo!(),
            TraitForTypeItemEthTemplate::AssociatedVal(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureBuilder {
    AssociatedType(TraitForTypeAssociatedTypeEtherealSignatureBuilder),
    Method(TraitForTypeMethodFnEtherealSignatureBuilder),
}

impl HasEthTemplate for TraitForTypeItemPath {
    type EthTemplate = TraitForTypeItemEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        trai_for_ty_item_ethereal_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_item_ethereal_signature_template(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> EtherealSignatureResult<TraitForTypeItemEthTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TraitForTypeItemDecTemplate::AssociatedFn(_) => todo!(),
        TraitForTypeItemDecTemplate::MethodFn(declarative_signature_template) => {
            TraitForTypeMethodFnEthTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TraitForTypeItemDecTemplate::AssociatedType(declarative_signature_template) => {
            TraitForTypeAssociatedTypeEthTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TraitForTypeItemDecTemplate::AssociatedVal(_) => todo!(),
    })
}
