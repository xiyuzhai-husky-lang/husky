mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureTemplate {
    AssociatedFn(TraitForTypeAssociatedFnEtherealSignatureTemplate),
    AssociatedVal(TraitForTypeAssociatedValEtherealSignatureTemplate),
    AssociatedType(TraitForTypeAssociatedTypeEtherealSignatureTemplate),
    MethodFn(TraitForTypeMethodFnEtherealSignatureTemplate),
}

impl TraitForTypeItemEtherealSignatureTemplate {
    pub(crate) fn inherit_partial_instantiation(
        self,
        db: &dyn EtherealSignatureDb,
        impl_block_template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ) -> TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated {
        match self {
            TraitForTypeItemEtherealSignatureTemplate::AssociatedType(item_template) => {
                item_template
                    .inherit_partial_instantiation(db, impl_block_template_partially_instantiated)
                    .into()
            }
            TraitForTypeItemEtherealSignatureTemplate::MethodFn(item_template) => item_template
                .inherit_partial_instantiation(db, impl_block_template_partially_instantiated)
                .into(),
            TraitForTypeItemEtherealSignatureTemplate::AssociatedFn(_) => todo!(),
            TraitForTypeItemEtherealSignatureTemplate::AssociatedVal(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated {
    AssociatedType(TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated),
    Method(TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated),
}

impl HasEtherealSignatureTemplate for TraitForTypeItemPath {
    type EtherealSignatureTemplate = TraitForTypeItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        trai_for_ty_item_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_item_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TraitForTypeItemPath,
) -> EtherealSignatureResult<TraitForTypeItemEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TraitForTypeItemDeclarativeSignatureTemplate::AssociatedFn(_) => todo!(),
        TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(declarative_signature_template) => {
            TraitForTypeMethodFnEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(
            declarative_signature_template,
        ) => TraitForTypeAssociatedTypeEtherealSignatureTemplate::from_declarative(
            db,
            path,
            declarative_signature_template,
        )?
        .into(),
        TraitForTypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
    })
}
