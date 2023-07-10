mod ethereal;
mod hollow;
mod solid;

use self::ethereal::*;
use self::hollow::*;
use self::solid::*;
use super::*;
use husky_coword::Ident;
use husky_ethereal_signature::{HasTypeItemTemplates, TypeItemEtherealSignatureTemplates};

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum ScopeResolutionDisambiguation {
    AssociatedFn(AssociatedFnFluffySignature),
}

impl FluffyTerm {
    pub fn disambiguate_scope_resolution(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        all_available_traits: &[()],
    ) -> FluffyTermMaybeResult<ScopeResolutionDisambiguation> {
        // todo: optimize for ethereal etc.
        let db = engine.db();
        match self.data(engine) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                arguments,
                ty_ethereal_term,
            } => match ty_path.ty_item_ethereal_signature_templates(db, ident) {
                JustOk(templates) => match templates {
                    TypeItemEtherealSignatureTemplates::AssociatedFn(templates) => {
                        let arguments: SmallVec<[_; 2]> = arguments.to_smallvec();
                        for template in templates.iter().copied() {
                            if let JustOk(signature) = ty_associated_fn_fluffy_signature(
                                engine,
                                template,
                                &arguments,
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
            FluffyTermData::TypeOntologyAtPlace {
                place,
                ty_path,
                refined_ty_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
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
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
        }
    }
}
