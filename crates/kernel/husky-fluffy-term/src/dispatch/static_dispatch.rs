mod ethereal;
mod hollow;
mod solid;

use super::*;
use husky_coword::Ident;
use husky_ethereal_signature::{HasTypeItemTemplates, TypeItemEtherealSignatureTemplates};

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum StaticDispatch {
    AssociatedFn(AssociatedFnFluffySignature),
    AssociatedGn,
}

impl FluffyTerm {
    pub fn static_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident: Ident,
        all_available_traits: &[()],
    ) -> FluffyTermMaybeResult<StaticDispatch> {
        // todo: optimize for ethereal etc.
        let db = engine.db();
        match self.data(engine) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match ty_path.ty_item_ethereal_signature_templates(db, ident) {
                JustOk(templates) => match templates {
                    TypeItemEtherealSignatureTemplates::AssociatedFn(templates) => {
                        let ty_arguments: SmallVec<[_; 2]> = ty_arguments.to_smallvec();
                        for template in templates.iter().copied() {
                            if let JustOk(signature) = ty_associated_fn_fluffy_signature(
                                engine,
                                expr_idx,
                                template,
                                &ty_arguments,
                                /* ad hoc */ &[],
                            ) {
                                return JustOk(signature.into());
                            }
                        }
                        Nothing
                    }
                    TypeItemEtherealSignatureTemplates::MethodFn(_) => todo!(),
                    TypeItemEtherealSignatureTemplates::MethodFunction(_) => todo!(),
                    TypeItemEtherealSignatureTemplates::MemoizedField(_) => todo!(),
                },
                JustErr(_) => todo!(),
                Nothing => todo!(),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
