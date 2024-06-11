mod ethereal;
mod hollow;
mod solid;

use super::*;
use assoc_static_var::AssocStaticVarFlySignature;
use husky_coword::Ident;
use husky_entity_path::path::major_item::trai::TraitPath;
use husky_entity_tree::node::HasAssocItemPaths;
use husky_eth_signature::signature::{
    assoc_item::{
        trai_item::TraitItemEthTemplate,
        ty_item::{HasTypeItemTemplates, TypeItemEthTemplates},
    },
    HasEthTemplate,
};
use husky_eth_term::term::application::TermFunctionReduced;
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum StaticDispatch {
    AssocRitchie(AssocRitchieFlySignature),
    AssocStaticVar(AssocStaticVarFlySignature),
}

impl StaticDispatch {
    pub fn ty_result(&self, engine: &mut impl FlyTermEngineMut) -> FlyTermResult<FlyTerm> {
        let db = engine.db();
        match self {
            StaticDispatch::AssocRitchie(ref signature) => Ok(signature.ty()),
            StaticDispatch::AssocStaticVar(ref signature) => Ok(signature.ty()),
        }
    }
}

impl FlyTerm {
    pub fn static_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
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
            } => match ty_path.ty_item_eth_templates(db, ident) {
                // we prioritize type item than trait item
                JustOk(templates) => match templates {
                    TypeItemEthTemplates::AssocRitchie(templates) => {
                        let dst_ty_arguments: SmallVec<[_; 2]> = ty_arguments.to_smallvec();
                        let signatures: Vec<_> = templates
                            .iter()
                            .copied()
                            .filter_map(|template| {
                                AssocRitchieFlySignature::from_ty_assoc_ritchie(
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
                                        ExpectSubtypeOrEqual::new(dst_ty_argument),
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
            FlyTermData::Trait { .. } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::SymbolicVariable {
                symbolic_variable, ..
            } => {
                let Ok(symbolic_variable_obvious_trais) =
                    engine.symbolic_variable_obvious_trais(symbolic_variable)
                else {
                    todo!()
                };
                match symbolic_variable_obvious_trais.len() {
                    0 => todo!(),
                    1 => {
                        let trai = symbolic_variable_obvious_trais[0];
                        let TermFunctionReduced::Trait(trai_path) =
                            trai.application_expansion(db).function()
                        else {
                            unreachable!()
                        };
                        let Some(&(_, trai_item_path)) =
                            trai_path.assoc_item_paths(db).get_entry(ident)
                        else {
                            todo!()
                        };
                        todo!()
                    }
                    _ => todo!(),
                }
            }
            FlyTermData::LambdaVariable { ty, index } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
            FlyTermData::MajorTypeVar(_) => todo!(),
        }
    }
    pub fn static_dispatch_as_trai(
        self,
        trai: Self,
        engine: &mut impl FlyTermEngineMut,
        syn_expr_idx: SynExprIdx,
        ident: Ident,
    ) -> FlyTermMaybeResult<StaticDispatch> {
        let db = engine.db();
        match self.data(engine) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Trait { .. } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::SymbolicVariable {
                symbolic_variable,
                ty,
            } => todo!(),
            FlyTermData::LambdaVariable { ty, index } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
            FlyTermData::MajorTypeVar(_) => {
                let trai_path: TraitPath = match trai.data(engine) {
                    FlyTermData::Trait { trai_path, .. } => trai_path,
                    _ => todo!(),
                };
                let Some(&(_, trai_item_path)) = trai_path.assoc_item_paths(db).get_entry(ident)
                else {
                    todo!()
                };
                JustOk(todo!())
            }
        }
    }
}
