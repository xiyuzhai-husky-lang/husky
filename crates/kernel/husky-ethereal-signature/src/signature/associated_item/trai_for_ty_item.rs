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
    AssociatedType(TraitForTypeAssociatedTypeEtherealSignatureTemplate),
}

impl TraitForTypeItemEtherealSignatureTemplate {
    pub(crate) fn inherit_partial_instantiation(
        self,
        db: &dyn EtherealSignatureDb,
        impl_block_template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ) -> TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated {
        match self {
            TraitForTypeItemEtherealSignatureTemplate::AssociatedType(item_template) => {
                let partial_instantiation = impl_block_template_partially_instantiated
                    .partial_instantiation(db)
                    .merge_with_item_generic_parameters(item_template.generic_parameters(db));
                TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated::new(
                    db,
                    item_template,
                    partial_instantiation,
                )
                .into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEtherealSignatureTemplatePartiallyInstantiated {
    AssociatedType(TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated),
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
        TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
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
