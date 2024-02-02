mod ethereal;
mod hollow;
mod solid;

use super::*;
use husky_coword::Ident;
use husky_eth_signature::{HasTypeItemTemplates, TypeItemEthTemplates};

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum StaticDispatch {
    AssociatedFn(AssociatedFnFlySignature),
    AssociatedGn,
}

impl FlyTerm {
    pub fn static_dispatch(
        self,
        engine: &mut impl FlyTermEngine,
        syn_expr_idx: SynExprIdx,
        ident: Ident,
        all_available_traits: &[()],
    ) -> FlyTermMaybeResult<StaticDispatch> {
        // todo: optimize for ethereal etc.
        let db = engine.db();
        match self.data(engine) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path,
                ty_arguments,
                ..
            } => match ty_path.ty_item_ethereal_signature_templates(db, ident) {
                JustOk(templates) => match templates {
                    TypeItemEthTemplates::AssociatedFn(templates) => {
                        let dst_ty_arguments: SmallVec<[_; 2]> = ty_arguments.to_smallvec();
                        let signatures: Vec<_> = templates
                            .iter()
                            .copied()
                            .filter_map(|template| {
                                ty_associated_fn_fluffy_signature(
                                    engine,
                                    syn_expr_idx,
                                    template,
                                    &dst_ty_arguments,
                                    /* ad hoc */ &[],
                                )
                                .ok()
                            })
                            .collect();
                        match signatures.len() {
                            0 => Nothing,
                            1 => {
                                let mut signatures = signatures;
                                let signature = signatures.pop().unwrap();
                                let FlyBaseTypeData::TypeOntology {
                                    ty_arguments: src_ty_arguments,
                                    ..
                                } = signature.self_ty().base_ty_data(engine)
                                else {
                                    unreachable!()
                                };
                                let src_ty_arguments: SmallVec<[_; 2]> =
                                    src_ty_arguments.to_smallvec();
                                debug_assert_eq!(src_ty_arguments.len(), dst_ty_arguments.len());
                                for (&src_ty_argument, &dst_ty_argument) in
                                    std::iter::zip(&src_ty_arguments, &dst_ty_arguments)
                                {
                                    engine.add_expectation(
                                        ExpectationSource::new_expr(syn_expr_idx),
                                        src_ty_argument,
                                        ExpectSubtype::new(dst_ty_argument),
                                    );
                                }
                                return JustOk(signature.into());
                            }
                            _ => todo!(),
                        }
                    }
                    TypeItemEthTemplates::MethodFn(_) => todo!(),
                    TypeItemEthTemplates::MethodFunction(_) => todo!(),
                    TypeItemEthTemplates::MemoizedField(_) => todo!(),
                },
                JustErr(_) => todo!(),
                Nothing => todo!(),
            },
            FlyTermData::Curry { .. } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Category(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }
}
